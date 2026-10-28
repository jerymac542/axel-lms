<?php

return [

    /*
    |--------------------------------------------------------------------------
    | Stateful Domains
    |--------------------------------------------------------------------------
    |
    | Requests from the following domains / hosts will receive stateful API
    | authentication cookies. Typically, these should include your local
    | and production domains which access your API via a frontend SPA.
    |
    */

    'stateful' => explode(',', env('SANCTUM_STATEFUL_DOMAINS', sprintf(
        '%s%s',
        'localhost,localhost:3000,127.0.0.1,127.0.0.1:8000,::1',
        env('APP_URL') ? ','.parse_url(env('APP_URL'), PHP_URL_HOST) : ''
    ))),

    /*
    |--------------------------------------------------------------------------
    | Sanctum Guards
    |--------------------------------------------------------------------------
    |
    | This array contains the authentication guards that will be checked when
    | Sanctum is trying to authenticate a request. If none of these guards
    | are able to authenticate the request, Sanctum will return an error.
    |
    */

    'guards' => ['web'],

    /*
    |--------------------------------------------------------------------------
    | Expiration Minutes
    |--------------------------------------------------------------------------
    |
    | This value controls the number of minutes until an issued token will be
    | considered expired. If this value is null, personal access tokens will
    | not expire.
    |
    */

    'expiration' => env('SANCTUM_TOKEN_EXPIRATION', 525600),

    /*
    |--------------------------------------------------------------------------
    | Token Prefix
    |--------------------------------------------------------------------------
    |
    | This value controls the prefix used for personal access tokens. This
    | allows you to easily identify tokens issued by your application.
    |
    */

    'prefix' => env('SANCTUM_TOKEN_PREFIX', 'axl_'),

    /*
    |--------------------------------------------------------------------------
    | Token Length
    |--------------------------------------------------------------------------
    |
    | This value controls the length of the random token generated when a
    | personal access token is created. You should choose a length that
    | provides adequate security while keeping the token manageable.
    |
    */

    'length' => env('SANCTUM_TOKEN_LENGTH', 40),

    /*
    |--------------------------------------------------------------------------
    | Hashing Algorithm
    |--------------------------------------------------------------------------
    |
    | This value controls the hashing algorithm used for token storage.
    |
    */

    'hashing_algorithm' => env('SANCTUM_HASHING_ALGORITHM', 'sha256'),

];