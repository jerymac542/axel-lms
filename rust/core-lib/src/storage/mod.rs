//! Storage
//!
//! S3/MinIO, database, and Redis abstractions for Apexos services.

use crate::config::{StorageConfig, StorageProvider, DatabaseConfig, RedisConfig};
use crate::domain::FileSize;
use crate::errors::{ErrorCode, Result};
use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::{
    config::{Credentials, SharedCredentialsProvider},
    operation::create_multipart_upload::CreateMultipartUploadOutput,
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart},
    Client as S3Client,
};
use bytes::Bytes;
use deadpool_postgres::{Config as PgConfig, Pool as PgPool, Runtime};
use futures::TryStreamExt;
use minio::s3::{Client as MinioClient, ClientBuilder as MinioClientBuilder};
use redis::{aio::ConnectionManager, Client as RedisClient, RedisResult};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;
use url::Url;
use uuid::Uuid;

/// Database pool implementation
pub struct PostgresPool {
    pool: PgPool,
}

impl PostgresPool {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let mut pg_config = PgConfig::new();
        pg_config.url = Some(config.url.expose_secret().clone());
        
        let pool = pg_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn acquire(&self) -> Result<PostgresConnection> {
        let client = self.pool.get().await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        
        Ok(PostgresConnection { client })
    }

    pub async fn health_check(&self) -> Result<()> {
        let client = self.pool.get().await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        
        client.execute("SELECT 1", &[]).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        
        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait::async_trait]
impl crate::http::DatabasePool for PostgresPool {
    async fn acquire(&self) -> Result<Box<dyn crate::http::DatabaseConnection>> {
        Ok(Box::new(self.acquire().await?))
    }

    async fn health_check(&self) -> Result<()> {
        self.health_check().await
    }
}

pub struct PostgresConnection {
    client: deadpool_postgres::Object,
}

#[async_trait::async_trait]
impl crate::http::DatabaseConnection for PostgresConnection {
    async fn execute(&self, query: &str) -> Result<u64> {
        let rows = self.client.execute(query, &[]).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        Ok(rows)
    }

    async fn query(&self, query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let rows = self.client.query(query, &[]).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        
        let mut results = Vec::new();
        for row in rows {
            let mut map = HashMap::new();
            for (i, column) in row.columns().iter().enumerate() {
                let value: serde_json::Value = serde_json::to_value(row.get::<_, Option<serde_json::Value>>(i)?).unwrap_or(serde_json::Value::Null);
                map.insert(column.name().to_string(), value);
            }
            results.push(map);
        }
        
        Ok(results)
    }

    async fn query_one(&self, query: &str) -> Result<Option<HashMap<String, serde_json::Value>>> {
        let row = self.client.query_opt(query, &[]).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        
        if let Some(row) = row {
            let mut map = HashMap::new();
            for (i, column) in row.columns().iter().enumerate() {
                let value: serde_json::Value = serde_json::to_value(row.get::<_, Option<serde_json::Value>>(i)?).unwrap_or(serde_json::Value::Null);
                map.insert(column.name().to_string(), value);
            }
            Ok(Some(map))
        } else {
            Ok(None)
        }
    }

    async fn begin_transaction(&self) -> Result<Box<dyn crate::http::Transaction>> {
        let transaction = self.client.transaction().await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        Ok(Box::new(PostgresTransaction { transaction: Some(transaction) }))
    }
}

pub struct PostgresTransaction {
    transaction: Option<tokio_postgres::Transaction<'static>>,
}

#[async_trait::async_trait]
impl crate::http::Transaction for PostgresTransaction {
    async fn execute(&self, query: &str) -> Result<u64> {
        let tx = self.transaction.as_ref().unwrap();
        let rows = tx.execute(query, &[]).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        Ok(rows)
    }

    async fn query(&self, query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let tx = self.transaction.as_ref().unwrap();
        let rows = tx.query(query, &[]).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        
        let mut results = Vec::new();
        for row in rows {
            let mut map = HashMap::new();
            for (i, column) in row.columns().iter().enumerate() {
                let value: serde_json::Value = serde_json::to_value(row.get::<_, Option<serde_json::Value>>(i)?).unwrap_or(serde_json::Value::Null);
                map.insert(column.name().to_string(), value);
            }
            results.push(map);
        }
        
        Ok(results)
    }

    async fn commit(mut self: Box<Self>) -> Result<()> {
        if let Some(tx) = self.transaction.take() {
            tx.commit().await
                .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        }
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> Result<()> {
        if let Some(tx) = self.transaction.take() {
            tx.rollback().await
                .map_err(|e| crate::errors::ApexosError::new(ErrorCode::DatabaseError, e.to_string()))?;
        }
        Ok(())
    }
}

/// Redis client implementation
pub struct RedisCache {
    manager: ConnectionManager,
}

impl RedisCache {
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let client = RedisClient::open(config.url.expose_secret().as_str())
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        
        let manager = ConnectionManager::new(client).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;

        Ok(Self { manager })
    }

    pub fn manager(&self) -> &ConnectionManager {
        &self.manager
    }

    pub async fn health_check(&self) -> Result<()> {
        let mut conn = self.manager.clone();
        redis::cmd("PING").query_async::<_, String>(&mut conn).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::http::CacheClient for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut conn = self.manager.clone();
        let value: Option<Vec<u8>> = redis::cmd("GET").arg(key).query_async(&mut conn).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        Ok(value)
    }

    async fn set(&self, key: &str, value: &[u8], ttl: Duration) -> Result<()> {
        let mut conn = self.manager.clone();
        redis::cmd("SET").arg(key).arg(value).arg("EX").arg(ttl.as_secs()).query_async(&mut conn).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.manager.clone();
        redis::cmd("DEL").arg(key).query_async(&mut conn).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.manager.clone();
        let exists: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        Ok(exists)
    }

    async fn increment(&self, key: &str, delta: i64) -> Result<i64> {
        let mut conn = self.manager.clone();
        let value: i64 = redis::cmd("INCRBY").arg(key).arg(delta).query_async(&mut conn).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::CacheError, e.to_string()))?;
        Ok(value)
    }
}

/// S3-compatible object store
pub struct S3ObjectStore {
    client: S3Client,
    bucket: String,
    config: StorageConfig,
}

impl S3ObjectStore {
    pub async fn new(config: &StorageConfig) -> Result<Self> {
        let sdk_config = match config.provider {
            StorageProvider::S3 => {
                let creds = Credentials::new(
                    config.access_key.expose_secret(),
                    config.secret_key.expose_secret(),
                    config.session_token.as_ref().map(|t| t.expose_secret().clone()),
                    None,
                    "apexos",
                );
                let provider = SharedCredentialsProvider::new(creds);
                
                aws_config::defaults(BehaviorVersion::latest())
                    .region(Region::new(config.region.clone()))
                    .credentials_provider(provider)
                    .load()
                    .await
            }
            StorageProvider::Minio => {
                let creds = Credentials::new(
                    config.access_key.expose_secret(),
                    config.secret_key.expose_secret(),
                    config.session_token.as_ref().map(|t| t.expose_secret().clone()),
                    None,
                    "apexos",
                );
                let provider = SharedCredentialsProvider::new(creds);
                
                let endpoint = config.endpoint.as_ref()
                    .ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "MinIO endpoint required"))?;
                
                aws_config::defaults(BehaviorVersion::latest())
                    .region(Region::new(config.region.clone()))
                    .endpoint_url(endpoint.to_string())
                    .credentials_provider(provider)
                    .load()
                    .await
            }
            _ => return Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Unsupported storage provider")),
        };

        let mut s3_config = aws_sdk_s3::config::Builder::from(&sdk_config);
        
        if config.use_path_style {
            s3_config = s3_config.force_path_style(true);
        }

        let client = S3Client::from_conf(s3_config.build());

        // Ensure bucket exists
        let _ = client.create_bucket()
            .bucket(&config.bucket)
            .send()
            .await;

        Ok(Self {
            client,
            bucket: config.bucket.clone(),
            config: config.clone(),
        })
    }

    pub fn client(&self) -> &S3Client {
        &self.client
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub async fn put(&self, key: &str, data: &[u8], content_type: &str) -> Result<String> {
        let body = ByteStream::from(Bytes::copy_from_slice(data));
        
        self.client.put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;

        Ok(key.to_string())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let result = self.client.get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await;

        match result {
            Ok(output) => {
                let data = output.body.collect().await
                    .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?
                    .into_bytes()
                    .to_vec();
                Ok(Some(data))
            }
            Err(e) => {
                if e.as_service_error().is_not_found() {
                    Ok(None)
                } else {
                    Err(crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))
                }
            }
        }
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        self.client.delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        let result = self.client.head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.as_service_error().is_not_found() {
                    Ok(false)
                } else {
                    Err(crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))
                }
            }
        }
    }

    pub async fn presign_get(&self, key: &str, ttl: Duration) -> Result<String> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(ttl)
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        
        let request = self.client.get_object()
            .bucket(&self.bucket)
            .key(key);
        
        let presigned = request.presign(presigning_config).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(presigned.uri().to_string())
    }

    pub async fn presign_put(&self, key: &str, ttl: Duration, content_type: &str) -> Result<String> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(ttl)
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        
        let request = self.client.put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type);
        
        let presigned = request.presign(presigning_config).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(presigned.uri().to_string())
    }

    pub async fn multipart_upload(&self, key: &str, content_type: &str) -> Result<CreateMultipartUploadOutput> {
        let result = self.client.create_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(result)
    }

    pub async fn upload_part(&self, key: &str, upload_id: &str, part_number: i32, data: &[u8]) -> Result<String> {
        let body = ByteStream::from(Bytes::copy_from_slice(data));
        
        let result = self.client.upload_part()
            .bucket(&self.bucket)
            .key(key)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(body)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(result.e_tag.unwrap_or_default())
    }

    pub async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        let completed = CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();

        self.client.complete_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .upload_id(upload_id)
            .multipart_upload(completed)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(())
    }

    pub async fn abort_multipart_upload(&self, key: &str, upload_id: &str) -> Result<()> {
        self.client.abort_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(())
    }

    pub async fn list_objects(&self, prefix: &str, max_keys: i32) -> Result<Vec<String>> {
        let result = self.client.list_objects_v2()
            .bucket(&self.bucket)
            .prefix(prefix)
            .max_keys(max_keys)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;

        let keys = result.contents()
            .unwrap_or_default()
            .iter()
            .filter_map(|o| o.key().map(|k| k.to_string()))
            .collect();

        Ok(keys)
    }

    pub async fn copy_object(&self, source_key: &str, dest_key: &str) -> Result<()> {
        let copy_source = format!("{}/{}", self.bucket, source_key);
        
        self.client.copy_object()
            .bucket(&self.bucket)
            .key(dest_key)
            .copy_source(copy_source)
            .send()
            .await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::http::ObjectStore for S3ObjectStore {
    async fn put(&self, key: &str, data: &[u8], content_type: &str) -> Result<String> {
        self.put(key, data, content_type).await
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        self.get(key).await
    }

    async fn delete(&self, key: &str) -> Result<()> {
        self.delete(key).await
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        self.exists(key).await
    }

    async fn presign_get(&self, key: &str, ttl: Duration) -> Result<String> {
        self.presign_get(key, ttl).await
    }

    async fn presign_put(&self, key: &str, ttl: Duration, content_type: &str) -> Result<String> {
        self.presign_put(key, ttl, content_type).await
    }
}

/// MinIO client (alternative implementation)
pub struct MinioObjectStore {
    client: MinioClient,
    bucket: String,
}

impl MinioObjectStore {
    pub async fn new(config: &StorageConfig) -> Result<Self> {
        let endpoint = config.endpoint.as_ref()
            .ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "MinIO endpoint required"))?;
        
        let client = MinioClientBuilder::new(endpoint.as_str())
            .access_key(config.access_key.expose_secret())
            .secret_key(config.secret_key.expose_secret())
            .region(config.region.clone())
            .build()
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;

        // Ensure bucket exists
        let exists = client.bucket_exists(config.bucket.as_str()).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        if !exists {
            client.make_bucket(config.bucket.as_str(), None).await
                .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        }

        Ok(Self {
            client,
            bucket: config.bucket.clone(),
        })
    }

    pub async fn put(&self, key: &str, data: &[u8], content_type: &str) -> Result<String> {
        let mut object = minio::s3::Object::new(key, data.len() as i64, minio::s3::PutObjectOpts::default().content_type(content_type));
        
        self.client.put_object(&self.bucket, &mut object).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        
        Ok(key.to_string())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let result = self.client.get_object(&self.bucket, key).await;
        
        match result {
            Ok(mut object) => {
                let mut data = Vec::new();
                object.read_to_end(&mut data).await
                    .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
                Ok(Some(data))
            }
            Err(e) => {
                if e.to_string().contains("NotFound") {
                    Ok(None)
                } else {
                    Err(crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))
                }
            }
        }
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        self.client.remove_object(&self.bucket, key).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))?;
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        self.client.object_exists(&self.bucket, key).await
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()))
    }
}

/// File upload handler
pub struct FileUploadHandler {
    object_store: Arc<dyn crate::http::ObjectStore>,
    max_size: FileSize,
    allowed_types: Vec<String>,
    temp_dir: String,
}

impl FileUploadHandler {
    pub fn new(
        object_store: Arc<dyn crate::http::ObjectStore>,
        max_size: FileSize,
        allowed_types: Vec<String>,
    ) -> Self {
        Self {
            object_store,
            max_size,
            allowed_types,
            temp_dir: std::env::var("TEMP").unwrap_or_else(|_| "/tmp".to_string()),
        }
    }

    pub async fn upload(&self, file: &mut FileUpload) -> Result<UploadResult> {
        // Validate file size
        if file.size > self.max_size.as_bytes() {
            return Err(crate::errors::ApexosError::new(ErrorCode::MediaTooLarge, "File too large"));
        }

        // Validate MIME type
        if !self.allowed_types.contains(&file.content_type) {
            return Err(crate::errors::ApexosError::new(ErrorCode::MediaTypeNotSupported, "File type not allowed"));
        }

        // Generate storage key
        let key = self.generate_key(&file.filename, &file.content_type);

        // Upload to object store
        let data = file.read_all().await?;
        self.object_store.put(&key, &data, &file.content_type).await?;

        // Generate presigned URL for access
        let url = self.object_store.presign_get(&key, Duration::from_secs(86400 * 365)).await?;

        Ok(UploadResult {
            key,
            url,
            filename: file.filename.clone(),
            content_type: file.content_type.clone(),
            size: file.size,
        })
    }

    pub async fn upload_multipart(&self, file: &mut FileUpload) -> Result<UploadResult> {
        if file.size <= self.max_size.as_bytes() {
            return self.upload(file).await;
        }

        // For large files, use multipart upload
        let key = self.generate_key(&file.filename, &file.content_type);
        
        // This would use the S3 multipart upload API
        // Simplified for now
        self.upload(file).await
    }

    fn generate_key(&self, filename: &str, content_type: &str) -> String {
        let extension = Path::new(filename).extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let uuid = Uuid::now_v7();
        let date = chrono::Utc::now().format("%Y/%m/%d");
        
        if extension.is_empty() {
            format!("uploads/{}/{}.bin", date, uuid)
        } else {
            format!("uploads/{}/{}.{}", date, uuid, extension)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileUpload {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
    pub data: Vec<u8>,
}

impl FileUpload {
    pub fn new(filename: String, content_type: String, data: Vec<u8>) -> Self {
        let size = data.len() as u64;
        Self { filename, content_type, size, data }
    }

    pub async fn read_all(&mut self) -> Result<Vec<u8>> {
        Ok(std::mem::take(&mut self.data))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult {
    pub key: String,
    pub url: String,
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

/// Media processing
pub mod media {
    use super::*;
    use crate::domain::MimeType;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MediaVariant {
        pub name: String,
        pub key: String,
        pub url: String,
        pub mime_type: MimeType,
        pub size: u64,
        pub width: Option<u32>,
        pub height: Option<u32>,
        pub duration: Option<f64>,
    }

    pub struct MediaProcessor {
        object_store: Arc<dyn crate::http::ObjectStore>,
        ffmpeg_path: String,
    }

    impl MediaProcessor {
        pub fn new(object_store: Arc<dyn crate::http::ObjectStore>) -> Self {
            Self {
                object_store,
                ffmpeg_path: "ffmpeg".to_string(),
            }
        }

        pub async fn process_video(&self, key: &str, variants: &[VideoVariant]) -> Result<Vec<MediaVariant>> {
            // Download original
            let data = self.object_store.get(key).await?
                .ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::MediaNotFound, "Original not found"))?;

            // Save to temp file
            let temp_input = format!("/tmp/{}", Uuid::now_v7());
            tokio::fs::write(&temp_input, &data).await?;

            let mut results = Vec::new();

            for variant in variants {
                let temp_output = format!("/tmp/{}_{}", Uuid::now_v7(), variant.name);
                
                // Run ffmpeg
                let mut cmd = tokio::process::Command::new(&self.ffmpeg_path);
                cmd.arg("-i").arg(&temp_input);
                
                for arg in &variant.ffmpeg_args {
                    cmd.arg(arg);
                }
                
                cmd.arg(&temp_output);
                
                let output = cmd.output().await
                    .map_err(|e| crate::errors::ApexosError::new(ErrorCode::MediaProcessingFailed, e.to_string()))?;

                if !output.status.success() {
                    return Err(crate::errors::ApexosError::new(ErrorCode::MediaProcessingFailed, 
                        String::from_utf8_lossy(&output.stderr).to_string()));
                }

                // Upload variant
                let variant_data = tokio::fs::read(&temp_output).await?;
                let variant_key = format!("{}_{}", key, variant.name);
                self.object_store.put(&variant_key, &variant_data, &variant.mime_type.0).await?;

                let url = self.object_store.presign_get(&variant_key, Duration::from_secs(86400 * 365)).await?;

                // Get dimensions (would need ffprobe)
                let (width, height) = self.get_dimensions(&temp_output).await?;

                results.push(MediaVariant {
                    name: variant.name.clone(),
                    key: variant_key,
                    url,
                    mime_type: variant.mime_type.clone(),
                    size: variant_data.len() as u64,
                    width,
                    height,
                    duration: variant.duration,
                });

                // Cleanup
                let _ = tokio::fs::remove_file(&temp_output).await;
            }

            // Cleanup input
            let _ = tokio::fs::remove_file(&temp_input).await;

            Ok(results)
        }

        pub async fn process_image(&self, key: &str, variants: &[ImageVariant]) -> Result<Vec<MediaVariant>> {
            // Similar to video but using image processing library
            // Placeholder
            Ok(vec![])
        }

        async fn get_dimensions(&self, path: &str) -> Result<(Option<u32>, Option<u32>)> {
            // Use ffprobe to get dimensions
            let output = tokio::process::Command::new("ffprobe")
                .args(["-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height", "-of", "csv=p=0", path])
                .output()
                .await?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = stdout.trim().split(',').collect();
                if parts.len() == 2 {
                    return Ok((parts[0].parse().ok(), parts[1].parse().ok()));
                }
            }
            Ok((None, None))
        }
    }

    #[derive(Debug, Clone)]
    pub struct VideoVariant {
        pub name: String,
        pub mime_type: MimeType,
        pub ffmpeg_args: Vec<String>,
        pub duration: Option<f64>,
    }

    #[derive(Debug, Clone)]
    pub struct ImageVariant {
        pub name: String,
        pub mime_type: MimeType,
        pub width: u32,
        pub height: u32,
        pub quality: u8,
    }
}

/// Storage factory
pub async fn create_object_store(config: &StorageConfig) -> Result<Arc<dyn crate::http::ObjectStore>> {
    match config.provider {
        StorageProvider::S3 | StorageProvider::Minio => {
            let store = S3ObjectStore::new(config).await?;
            Ok(Arc::new(store))
        }
        _ => Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Unsupported storage provider")),
    }
}

pub async fn create_database_pool(config: &DatabaseConfig) -> Result<Arc<dyn crate::http::DatabasePool>> {
    let pool = PostgresPool::new(config).await?;
    Ok(Arc::new(pool))
}

pub async fn create_cache_client(config: &RedisConfig) -> Result<Arc<dyn crate::http::CacheClient>> {
    let client = RedisCache::new(config).await?;
    Ok(Arc::new(client))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_upload_validation() {
        let allowed = vec!["image/jpeg".to_string(), "image/png".to_string()];
        let handler = FileUploadHandler::new(
            Arc::new(MockObjectStore),
            FileSize::mb(10),
            allowed,
        );
        
        // Would test with mock
    }
}

// Mock for testing
struct MockObjectStore;

#[async_trait::async_trait]
impl crate::http::ObjectStore for MockObjectStore {
    async fn put(&self, _key: &str, _data: &[u8], _content_type: &str) -> Result<String> {
        Ok("test-key".to_string())
    }

    async fn get(&self, _key: &str) -> Result<Option<Vec<u8>>> {
        Ok(Some(b"test".to_vec()))
    }

    async fn delete(&self, _key: &str) -> Result<()> {
        Ok(())
    }

    async fn exists(&self, _key: &str) -> Result<bool> {
        Ok(true)
    }

    async fn presign_get(&self, _key: &str, _ttl: Duration) -> Result<String> {
        Ok("https://example.com/test".to_string())
    }

    async fn presign_put(&self, _key: &str, _ttl: Duration, _content_type: &str) -> Result<String> {
        Ok("https://example.com/test".to_string())
    }
}