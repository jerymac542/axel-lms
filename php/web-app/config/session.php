<?php

return [

    /*
    |--------------------------------------------------------------------------
    | Default Session Driver
    |--------------------------------------------------------------------------
    |
    | This option controls the default session driver that will be used by
    | the framework. The "database" driver is recommended for production
    | environments that use a load balancer or multiple application servers.
    |
    | Supported drivers: "file", "cookie", "database", "apc", "memcached",
    | "redis", "dynamodb", "array"
    |
    */

    'driver' => env('SESSION_DRIVER', 'redis'),

    /*
    |--------------------------------------------------------------------------
    | Session Lifetime
    |--------------------------------------------------------------------------
    |
    | This option controls the number of minutes that a session will be
    | allowed to remain idle before it is considered expired. If you want
    | the session to expire when the user closes their browser, set the
    | "expire_on_close" option to true.
    |
    */

    'lifetime' => env('SESSION_LIFETIME', 120),

    'expire_on_close' => false,

    /*
    |--------------------------------------------------------------------------
    | Session Encryption
    |--------------------------------------------------------------------------
    |
    | This option controls whether the session data should be encrypted
    | before it is stored in the session storage. Encryption provides
    | additional security by preventing tampering with session data.
    |
    */

    'encrypt' => false,

    /*
    |--------------------------------------------------------------------------
    | Session File Location
    |--------------------------------------------------------------------------
    |
    | When using the "file" session driver, this option determines the
    | path where session files will be stored.
    |
    */

    'files' => storage_path('framework/sessions'),

    /*
    |--------------------------------------------------------------------------
    | Session Database Connection
    |--------------------------------------------------------------------------
    |
    | When using the "database" or "redis" session drivers, you may specify
    | a specific database connection that should be used to store session
    | data. This option can be set to any of your configured database
    | connections.
    |
    */

    'connection' => env('SESSION_CONNECTION', 'redis'),

    /*
    |--------------------------------------------------------------------------
    | Session Database Table
    |--------------------------------------------------------------------------
    |
    | When using the "database" session driver, you may specify the table
    | that should be used to store session data.
    |
    */

    'table' => 'sessions',

    /*
    |--------------------------------------------------------------------------
    | Session Cache Store
    |--------------------------------------------------------------------------
    |
    | When using the "redis" session driver, you may specify the cache store
    | that should be used to store session data. This option can be set to
    | any of your configured cache stores.
    |
    */

    'store' => env('SESSION_STORE'),

    /*
    |--------------------------------------------------------------------------
    | Session Sweeping Lottery
    |--------------------------------------------------------------------------
    |
    | Some session drivers require manual sweeping of expired session data
    | from their storage. The "lottery" setting defines the probability that
    | the garbage collector will run on any given request.
    |
    */

    'lottery' => [2, 100],

    /*
    |--------------------------------------------------------------------------
    | Session Cookie Name
    |--------------------------------------------------------------------------
    |
    | This option determines the name of the cookie that is used to store
    | the session ID. You should change this to something unique for your
    | application.
    |
    */

    'cookie' => env('SESSION_COOKIE', 'axel_session'),

    /*
    |--------------------------------------------------------------------------
    | Session Cookie Path
    |--------------------------------------------------------------------------
    |
    | The session cookie path determines the path for which the cookie will
    | be regarded as valid. Typically, this will be the root path of your
    | application.
    |
    */

    'path' => '/',

    /*
    |--------------------------------------------------------------------------
    | Session Cookie Domain
    |--------------------------------------------------------------------------
    |
    | Here you may change the domain of the cookie used to identify a session
    | in your application. This will determine which domains the cookie is
    | available to. Leave this empty for the default behavior.
    |
    */

    'domain' => env('SESSION_DOMAIN'),

    /*
    |--------------------------------------------------------------------------
    | HTTPS Only Cookies
    |--------------------------------------------------------------------------
    |
    | By setting this option to true, you are instructing the browser to only
    | send the session cookie over HTTPS connections. This should be enabled
    | in production environments where HTTPS is available.
    |
    */

    'secure' => env('SESSION_SECURE_COOKIE', true),

    /*
    |--------------------------------------------------------------------------
    | HTTP Only Cookies
    |--------------------------------------------------------------------------
    |
    | By setting this option to true, you are preventing the session cookie
    | from being accessed via JavaScript. This helps mitigate XSS attacks.
    |
    */

    'http_only' => true,

    /*
    |--------------------------------------------------------------------------
    | Same-Site Cookies
    |--------------------------------------------------------------------------
    |
    | This option determines how the cookie behaves when cross-site requests
    | are made. Possible values: "lax", "strict", "none", or "null".
    |
    */

    'same_site' => 'lax',

    /*
    |--------------------------------------------------------------------------
    | Session Partitioned Cookies
    |--------------------------------------------------------------------------
    |
    | When this option is enabled, the session cookie will be partitioned
    | by top-level site, providing additional privacy protection.
    |
    */

    'partitioned' => false,

];