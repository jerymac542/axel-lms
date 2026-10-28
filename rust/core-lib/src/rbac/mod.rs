//! RBAC (Role-Based Access Control)
//!
//! Permissions, roles, policies, and authorization engine for Apexos services.

use crate::domain::{RoleId, PermissionId, UserId, OrganizationId, TenantId, AggregateId};
use crate::errors::{ErrorCode, Result};
use crate::tenancy::TenantContext;
use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

/// Permission definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission {
    pub id: PermissionId,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
    pub scope: PermissionScope,
    pub conditions: Vec<PermissionCondition>,
    pub created_at: crate::domain::Timestamp,
    pub updated_at: crate::domain::Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScope {
    /// Global permission (system-wide)
    Global,
    /// Tenant-scoped permission
    Tenant,
    /// Organization-scoped permission
    Organization,
    /// Course-scoped permission
    Course,
    /// Resource-specific permission
    Resource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PermissionCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Exists,
    NotExists,
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub description: String,
    pub scope: RoleScope,
    pub permissions: HashSet<PermissionId>,
    pub inherits: HashSet<RoleId>,
    pub is_system: bool,
    pub created_at: crate::domain::Timestamp,
    pub updated_at: crate::domain::Timestamp,
    pub deleted_at: Option<crate::domain::Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleScope {
    Global,
    Tenant,
    Organization,
    Course,
}

/// Policy for complex authorization rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effect: PolicyEffect,
    pub rules: Vec<PolicyRule>,
    pub priority: i32,
    pub enabled: bool,
    pub created_at: crate::domain::Timestamp,
    pub updated_at: crate::domain::Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub condition: PolicyCondition,
    pub effect: PolicyEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    pub subject: SubjectMatcher,
    pub resource: ResourceMatcher,
    pub action: ActionMatcher,
    pub context: ContextMatcher,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectMatcher {
    pub user_ids: Option<HashSet<UserId>>,
    pub roles: Option<HashSet<RoleId>>,
    pub groups: Option<HashSet<String>>,
    pub attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMatcher {
    pub types: Option<HashSet<String>>,
    pub ids: Option<HashSet<AggregateId>>,
    pub attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMatcher {
    pub actions: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMatcher {
    pub tenant_ids: Option<HashSet<TenantId>>,
    pub organization_ids: Option<HashSet<OrganizationId>>,
    pub time_range: Option<TimeRange>,
    pub ip_ranges: Option<Vec<String>>,
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Option<crate::domain::Timestamp>,
    pub end: Option<crate::domain::Timestamp>,
    pub timezone: Option<String>,
    pub days_of_week: Option<Vec<u8>>, // 0-6, Sunday-Saturday
    pub hours: Option<(u8, u8)>, // (start_hour, end_hour)
}

/// Authorization subject (user, service, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: UserId,
    pub tenant_id: TenantId,
    pub organization_ids: HashSet<OrganizationId>,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<PermissionId>,
    pub groups: HashSet<String>,
    pub attributes: HashMap<String, serde_json::Value>,
    pub mfa_verified: bool,
    pub authenticated_at: crate::domain::Timestamp,
}

impl Subject {
    pub fn new(user_id: UserId, tenant_id: TenantId) -> Self {
        Self {
            id: user_id,
            tenant_id,
            organization_ids: HashSet::new(),
            roles: HashSet::new(),
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: HashMap::new(),
            mfa_verified: false,
            authenticated_at: crate::domain::Timestamp::now(),
        }
    }

    pub fn with_organization(mut self, org_id: OrganizationId) -> Self {
        self.organization_ids.insert(org_id);
        self
    }

    pub fn with_role(mut self, role_id: RoleId) -> Self {
        self.roles.insert(role_id);
        self
    }

    pub fn with_permission(mut self, perm_id: PermissionId) -> Self {
        self.permissions.insert(perm_id);
        self
    }

    pub fn with_group(mut self, group: String) -> Self {
        self.groups.insert(group);
        self
    }

    pub fn with_attribute(mut self, key: String, value: serde_json::Value) -> Self {
        self.attributes.insert(key, value);
        self
    }

    pub fn with_mfa(mut self, verified: bool) -> Self {
        self.mfa_verified = verified;
        self
    }
}

/// Authorization resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub type_: String,
    pub id: AggregateId,
    pub tenant_id: TenantId,
    pub organization_id: Option<OrganizationId>,
    pub owner_id: Option<UserId>,
    pub attributes: HashMap<String, serde_json::Value>,
}

impl Resource {
    pub fn new(type_: impl Into<String>, id: AggregateId, tenant_id: TenantId) -> Self {
        Self {
            type_: type_.into(),
            id,
            tenant_id,
            organization_id: None,
            owner_id: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_organization(mut self, org_id: OrganizationId) -> Self {
        self.organization_id = Some(org_id);
        self
    }

    pub fn with_owner(mut self, user_id: UserId) -> Self {
        self.owner_id = Some(user_id);
        self
    }

    pub fn with_attribute(mut self, key: String, value: serde_json::Value) -> Self {
        self.attributes.insert(key, value);
        self
    }
}

/// Authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub subject: Subject,
    pub resource: Resource,
    pub action: String,
    pub context: AuthorizationContext,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationContext {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: crate::domain::Timestamp,
    pub correlation_id: Option<crate::domain::CorrelationId>,
    pub custom: HashMap<String, serde_json::Value>,
}

/// Authorization decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationDecision {
    pub allowed: bool,
    pub reason: String,
    pub matched_policies: Vec<String>,
    pub denied_policies: Vec<String>,
    pub required_permissions: Vec<PermissionId>,
    pub required_roles: Vec<RoleId>,
}

impl AuthorizationDecision {
    pub fn allow(reason: impl Into<String>) -> Self {
        Self {
            allowed: true,
            reason: reason.into(),
            matched_policies: vec![],
            denied_policies: vec![],
            required_permissions: vec![],
            required_roles: vec![],
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            allowed: false,
            reason: reason.into(),
            matched_policies: vec![],
            denied_policies: vec![],
            required_permissions: vec![],
            required_roles: vec![],
        }
    }
}

/// Authorization engine
pub struct AuthorizationEngine {
    permissions: DashMap<PermissionId, Permission>,
    roles: DashMap<RoleId, Role>,
    policies: DashMap<String, Policy>,
    role_permissions: DashMap<RoleId, HashSet<PermissionId>>,
    permission_cache: DashMap<String, AuthorizationDecision>,
}

impl AuthorizationEngine {
    pub fn new() -> Self {
        Self {
            permissions: DashMap::new(),
            roles: DashMap::new(),
            policies: DashMap::new(),
            role_permissions: DashMap::new(),
            permission_cache: DashMap::new(),
        }
    }

    /// Register a permission
    pub fn register_permission(&self, permission: Permission) {
        self.permissions.insert(permission.id, permission);
    }

    /// Register a role
    pub fn register_role(&self, role: Role) {
        // Compute effective permissions including inherited roles
        let effective_perms = self.compute_effective_permissions(&role);
        self.role_permissions.insert(role.id, effective_perms);
        self.roles.insert(role.id, role);
    }

    /// Register a policy
    pub fn register_policy(&self, policy: Policy) {
        self.policies.insert(policy.id.clone(), policy);
    }

    /// Compute effective permissions for a role (including inheritance)
    fn compute_effective_permissions(&self, role: &Role) -> HashSet<PermissionId> {
        let mut perms = role.permissions.clone();
        
        for inherit_id in &role.inherits {
            if let Some(parent_role) = self.roles.get(inherit_id) {
                if let Some(parent_perms) = self.role_permissions.get(inherit_id) {
                    perms.extend(parent_perms.iter().cloned());
                }
            }
        }
        
        perms
    }

    /// Check if subject has permission
    pub fn has_permission(&self, subject: &Subject, permission_id: PermissionId) -> bool {
        // Direct permission
        if subject.permissions.contains(&permission_id) {
            return true;
        }

        // Role-based permission
        for role_id in &subject.roles {
            if let Some(perms) = self.role_permissions.get(role_id) {
                if perms.contains(&permission_id) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if subject has role
    pub fn has_role(&self, subject: &Subject, role_id: RoleId) -> bool {
        subject.roles.contains(&role_id)
    }

    /// Check if subject has any of the roles
    pub fn has_any_role(&self, subject: &Subject, role_ids: &[RoleId]) -> bool {
        role_ids.iter().any(|r| subject.roles.contains(r))
    }

    /// Authorize a request
    pub fn authorize(&self, request: AuthorizationRequest) -> AuthorizationDecision {
        let cache_key = self.cache_key(&request);
        
        if let Some(cached) = self.permission_cache.get(&cache_key) {
            return cached.clone();
        }

        let decision = self.evaluate_request(&request);
        self.permission_cache.insert(cache_key, decision.clone());
        decision
    }

    fn cache_key(&self, request: &AuthorizationRequest) -> String {
        format!(
            "{}:{}:{}:{}",
            request.subject.id,
            request.resource.type_,
            request.resource.id,
            request.action
        )
    }

    fn evaluate_request(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
        let mut matched_policies = Vec::new();
        let mut denied_policies = Vec::new();
        let mut final_decision: Option<bool> = None;

        // Evaluate policies in priority order
        let mut policies: Vec<_> = self.policies.iter().map(|p| p.value().clone()).collect();
        policies.sort_by_key(|p| -p.priority); // Higher priority first

        for policy in policies {
            if !policy.enabled {
                continue;
            }

            if self.policy_matches(&policy, &request) {
                match policy.effect {
                    PolicyEffect::Allow => {
                        final_decision = Some(true);
                        matched_policies.push(policy.id);
                    }
                    PolicyEffect::Deny => {
                        final_decision = Some(false);
                        denied_policies.push(policy.id);
                    }
                }
            }
        }

        // If no policy matched, check direct permissions
        if final_decision.is_none() {
            let has_perm = self.has_permission(&request.subject, PermissionId::from_str(&request.action).unwrap_or_default());
            
            if has_perm {
                return AuthorizationDecision::allow("Permission granted via direct assignment");
            } else {
                return AuthorizationDecision::deny("No matching policy or permission");
            }
        }

        if final_decision.unwrap() {
            AuthorizationDecision::allow("Policy allowed")
        } else {
            AuthorizationDecision::deny("Policy denied")
        }
    }

    fn policy_matches(&self, policy: &Policy, request: &AuthorizationRequest) -> bool {
        for rule in &policy.rules {
            if self.condition_matches(&rule.condition, request) {
                return true;
            }
        }
        false
    }

    fn condition_matches(&self, condition: &PolicyCondition, request: &AuthorizationRequest) -> bool {
        // Subject matching
        if let Some(user_ids) = &condition.subject.user_ids {
            if !user_ids.contains(&request.subject.id) {
                return false;
            }
        }
        if let Some(roles) = &condition.subject.roles {
            if !request.subject.roles.iter().any(|r| roles.contains(r)) {
                return false;
            }
        }
        if let Some(groups) = &condition.subject.groups {
            if !request.subject.groups.iter().any(|g| groups.contains(g)) {
                return false;
            }
        }
        for (key, value) in &condition.subject.attributes {
            if request.subject.attributes.get(key) != Some(value) {
                return false;
            }
        }

        // Resource matching
        if let Some(types) = &condition.resource.types {
            if !types.contains(&request.resource.type_) {
                return false;
            }
        }
        if let Some(ids) = &condition.resource.ids {
            if !ids.contains(&request.resource.id) {
                return false;
            }
        }
        for (key, value) in &condition.resource.attributes {
            if request.resource.attributes.get(key) != Some(value) {
                return false;
            }
        }

        // Action matching
        if !condition.action.actions.contains(&request.action) {
            return false;
        }

        // Context matching
        if let Some(tenant_ids) = &condition.context.tenant_ids {
            if !tenant_ids.contains(&request.subject.tenant_id) {
                return false;
            }
        }
        if let Some(org_ids) = &condition.context.organization_ids {
            if !request.subject.organization_ids.iter().any(|o| org_ids.contains(o)) {
                return false;
            }
        }
        if let Some(time_range) = &condition.context.time_range {
            let now = crate::domain::Timestamp::now();
            if let Some(start) = time_range.start {
                if now < start {
                    return false;
                }
            }
            if let Some(end) = time_range.end {
                if now > end {
                    return false;
                }
            }
        }
        if let Some(ip_ranges) = &condition.context.ip_ranges {
            if let Some(ip) = &request.context.ip {
                let matched = ip_ranges.iter().any(|range| {
                    // Simplified CIDR check
                    ip.starts_with(&range.split('/').next().unwrap_or(""))
                });
                if !matched {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Clear permission cache
    pub fn clear_cache(&self) {
        self.permission_cache.clear();
    }

    /// Get all permissions
    pub fn list_permissions(&self) -> Vec<Permission> {
        self.permissions.iter().map(|p| p.value().clone()).collect()
    }

    /// Get all roles
    pub fn list_roles(&self) -> Vec<Role> {
        self.roles.iter().map(|r| r.value().clone()).collect()
    }

    /// Get all policies
    pub fn list_policies(&self) -> Vec<Policy> {
        self.policies.iter().map(|p| p.value().clone()).collect()
    }
}

impl Default for AuthorizationEngine {
    fn default() -> Self {
        let engine = Self::new();
        
        // Register system permissions
        let system_perms = vec![
            // User management
            ("user.create", "Create users", "user", "create"),
            ("user.read", "Read users", "user", "read"),
            ("user.update", "Update users", "user", "update"),
            ("user.delete", "Delete users", "user", "delete"),
            ("user.list", "List users", "user", "list"),
            
            // Role management
            ("role.create", "Create roles", "role", "create"),
            ("role.read", "Read roles", "role", "read"),
            ("role.update", "Update roles", "role", "update"),
            ("role.delete", "Delete roles", "role", "delete"),
            ("role.assign", "Assign roles", "role", "assign"),
            
            // Organization management
            ("organization.create", "Create organizations", "organization", "create"),
            ("organization.read", "Read organizations", "organization", "read"),
            ("organization.update", "Update organizations", "organization", "update"),
            ("organization.delete", "Delete organizations", "organization", "delete"),
            
            // Course management
            ("course.create", "Create courses", "course", "create"),
            ("course.read", "Read courses", "course", "read"),
            ("course.update", "Update courses", "course", "update"),
            ("course.delete", "Delete courses", "course", "delete"),
            ("course.publish", "Publish courses", "course", "publish"),
            ("course.manage", "Manage courses", "course", "manage"),
            
            // Enrollment
            ("enrollment.create", "Create enrollments", "enrollment", "create"),
            ("enrollment.read", "Read enrollments", "enrollment", "read"),
            ("enrollment.update", "Update enrollments", "enrollment", "update"),
            ("enrollment.delete", "Delete enrollments", "enrollment", "delete"),
            
            // Assignment
            ("assignment.create", "Create assignments", "assignment", "create"),
            ("assignment.read", "Read assignments", "assignment", "read"),
            ("assignment.update", "Update assignments", "assignment", "update"),
            ("assignment.delete", "Delete assignments", "assignment", "delete"),
            ("assignment.grade", "Grade assignments", "assignment", "grade"),
            
            // Submission
            ("submission.create", "Create submissions", "submission", "create"),
            ("submission.read", "Read submissions", "submission", "read"),
            ("submission.update", "Update submissions", "submission", "update"),
            ("submission.delete", "Delete submissions", "submission", "delete"),
            
            // Media
            ("media.upload", "Upload media", "media", "upload"),
            ("media.read", "Read media", "media", "read"),
            ("media.delete", "Delete media", "media", "delete"),
            ("media.process", "Process media", "media", "process"),
            
            // Analytics
            ("analytics.read", "Read analytics", "analytics", "read"),
            ("analytics.export", "Export analytics", "analytics", "export"),
            
            // Settings
            ("settings.read", "Read settings", "settings", "read"),
            ("settings.update", "Update settings", "settings", "update"),
            
            // LTI
            ("lti.launch", "Launch LTI tools", "lti", "launch"),
            ("lti.configure", "Configure LTI", "lti", "configure"),
            
            // SCORM
            ("scorm.upload", "Upload SCORM", "scorm", "upload"),
            ("scorm.play", "Play SCORM", "scorm", "play"),
            ("scorm.manage", "Manage SCORM", "scorm", "manage"),
            
            // AI
            ("ai.inference", "Run AI inference", "ai", "inference"),
            ("ai.train", "Train AI models", "ai", "train"),
            ("ai.manage", "Manage AI", "ai", "manage"),
            
            // Admin
            ("admin.access", "Access admin panel", "admin", "access"),
            ("admin.users", "Manage users", "admin", "users"),
            ("admin.tenants", "Manage tenants", "admin", "tenants"),
            ("admin.system", "System administration", "admin", "system"),
        ];

        for (name, desc, resource, action) in system_perms {
            let perm = Permission {
                id: PermissionId::new(),
                name: name.to_string(),
                description: desc.to_string(),
                resource: resource.to_string(),
                action: action.to_string(),
                scope: PermissionScope::Organization,
                conditions: vec![],
                created_at: crate::domain::Timestamp::now(),
                updated_at: crate::domain::Timestamp::now(),
            };
            engine.register_permission(perm);
        }

        // Register system roles
        let admin_role = Role {
            id: RoleId::new(),
            name: "admin".to_string(),
            description: "System administrator".to_string(),
            scope: RoleScope::Global,
            permissions: engine.permissions.iter().map(|p| p.key().clone()).collect(),
            inherits: HashSet::new(),
            is_system: true,
            created_at: crate::domain::Timestamp::now(),
            updated_at: crate::domain::Timestamp::now(),
            deleted_at: None,
        };
        engine.register_role(admin_role);

        let org_admin_role = Role {
            id: RoleId::new(),
            name: "org_admin".to_string(),
            description: "Organization administrator".to_string(),
            scope: RoleScope::Organization,
            permissions: engine.permissions.iter()
                .filter(|p| matches!(p.scope, PermissionScope::Organization | PermissionScope::Tenant))
                .map(|p| p.key().clone())
                .collect(),
            inherits: HashSet::new(),
            is_system: true,
            created_at: crate::domain::Timestamp::now(),
            updated_at: crate::domain::Timestamp::now(),
            deleted_at: None,
        };
        engine.register_role(org_admin_role);

        let instructor_role = Role {
            id: RoleId::new(),
            name: "instructor".to_string(),
            description: "Course instructor".to_string(),
            scope: RoleScope::Course,
            permissions: engine.permissions.iter()
                .filter(|p| matches!(p.scope, PermissionScope::Course | PermissionScope::Resource))
                .map(|p| p.key().clone())
                .collect(),
            inherits: HashSet::new(),
            is_system: true,
            created_at: crate::domain::Timestamp::now(),
            updated_at: crate::domain::Timestamp::now(),
            deleted_at: None,
        };
        engine.register_role(instructor_role);

        let student_role = Role {
            id: RoleId::new(),
            name: "student".to_string(),
            description: "Course student".to_string(),
            scope: RoleScope::Course,
            permissions: engine.permissions.iter()
                .filter(|p| p.action == "read" || p.action == "create")
                .map(|p| p.key().clone())
                .collect(),
            inherits: HashSet::new(),
            is_system: true,
            created_at: crate::domain::Timestamp::now(),
            updated_at: crate::domain::Timestamp::now(),
            deleted_at: None,
        };
        engine.register_role(student_role);

        engine
    }
}

/// Policy-based authorization middleware
pub mod middleware {
    use super::*;
    use axum::{
        extract::{Request, State},
        http::StatusCode,
        middleware::Next,
        response::Response,
    };
    use std::sync::Arc;

    pub async fn authorization_middleware(
        State(engine): State<Arc<AuthorizationEngine>>,
        request: Request,
        next: Next,
    ) -> Result<Response, crate::errors::ApexosError> {
        // Extract subject from request extensions (set by auth middleware)
        let subject = request.extensions().get::<Subject>().cloned()
            .ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::Unauthorized, "Not authenticated"))?;

        // Extract resource info from path
        let (resource_type, resource_id) = extract_resource_info(&request);
        
        let resource = Resource::new(resource_type, resource_id, subject.tenant_id);

        let auth_request = AuthorizationRequest {
            subject,
            resource,
            action: request.method().to_string().to_lowercase(),
            context: AuthorizationContext {
                ip: extract_client_ip(&request),
                user_agent: request.headers().get("user-agent").and_then(|v| v.to_str().ok()).map(String::from),
                timestamp: crate::domain::Timestamp::now(),
                correlation_id: request.extensions().get::<crate::domain::CorrelationId>().cloned(),
                custom: HashMap::new(),
            },
        };

        let decision = engine.authorize(auth_request);

        if !decision.allowed {
            return Err(crate::errors::ApexosError::new(ErrorCode::Forbidden, decision.reason));
        }

        Ok(next.run(request).await)
    }

    fn extract_resource_info(request: &Request) -> (String, crate::domain::AggregateId) {
        let path = request.uri().path();
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        
        // Try to extract resource type and ID from path
        // e.g., /api/v1/courses/{id} -> ("course", id)
        if parts.len() >= 3 {
            let resource_type = parts[1].trim_end_matches('s').to_string(); // Remove plural 's'
            if let Ok(id) = crate::domain::AggregateId::from_str(parts[2]) {
                return (resource_type, id);
            }
        }

        ("unknown".to_string(), crate::domain::AggregateId::new())
    }

    fn extract_client_ip(request: &Request) -> Option<String> {
        request.headers().get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
            .or_else(|| request.headers().get("x-real-ip").and_then(|v| v.to_str().ok()).map(String::from))
    }
}

/// Authorization macros
#[macro_export]
macro_rules! require_permission {
    ($engine:expr, $subject:expr, $perm:expr) => {
        if !$engine.has_permission($subject, $perm) {
            return Err(crate::errors::ApexosError::new(
                crate::errors::ErrorCode::Forbidden,
                format!("Permission required: {}", $perm)
            ));
        }
    };
}

#[macro_export]
macro_rules! require_role {
    ($engine:expr, $subject:expr, $role:expr) => {
        if !$engine.has_role($subject, $role) {
            return Err(crate::errors::ApexosError::new(
                crate::errors::ErrorCode::Forbidden,
                format!("Role required: {}", $role)
            ));
        }
    };
}

#[macro_export]
macro_rules! authorize {
    ($engine:expr, $subject:expr, $resource:expr, $action:expr) => {
        let request = AuthorizationRequest {
            subject: $subject,
            resource: $resource,
            action: $action.to_string(),
            context: AuthorizationContext::default(),
        };
        let decision = $engine.authorize(request);
        if !decision.allowed {
            return Err(crate::errors::ApexosError::new(
                crate::errors::ErrorCode::Forbidden,
                decision.reason
            ));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_registration() {
        let engine = AuthorizationEngine::new();
        assert!(!engine.permissions.is_empty());
        assert!(!engine.roles.is_empty());
    }

    #[test]
    fn role_permissions() {
        let engine = AuthorizationEngine::new();
        
        // Admin should have all permissions
        let admin_role = engine.roles.iter().find(|r| r.name == "admin").unwrap();
        assert_eq!(admin_role.permissions.len(), engine.permissions.len());
    }

    #[test]
    fn authorization_decision() {
        let engine = AuthorizationEngine::new();
        
        let subject = Subject::new(UserId::new(), TenantId::new())
            .with_role(*engine.roles.iter().find(|r| r.name == "admin").unwrap().key());
        
        let resource = Resource::new("user", AggregateId::new(), TenantId::new());
        
        let request = AuthorizationRequest {
            subject,
            resource,
            action: "read".to_string(),
            context: AuthorizationContext::default(),
        };
        
        let decision = engine.authorize(request);
        assert!(decision.allowed);
    }
}