<?php

return [

    /*
    |--------------------------------------------------------------------------
    | Axel LMS Branding Configuration
    |--------------------------------------------------------------------------
    |
    | Centralized branding configuration for Axel LMS, Apexos, and Apex Academy.
    | All logos, colors, and brand assets are defined here for consistent usage.
    |
    */

    'brands' => [

        /*
         * Axel LMS - Main Product Brand
         */
        'axel' => [
            'name' => 'Axel LMS',
            'tagline' => 'The enterprise learning platform for modern organizations',
            'description' => 'Build, deliver, and scale world-class learning experiences.',
            
            'logos' => [
                'primary' => [
                    'light' => '/images/logos/axel-lms-logo.png',
                    'dark' => '/images/logos/axel-lms-logo.png',
                    'width' => 160,
                    'height' => 40,
                ],
                'icon' => [
                    'light' => '/images/logos/axel-lms-icon.png',
                    'dark' => '/images/logos/axel-lms-icon.png',
                    'width' => 32,
                    'height' => 32,
                ],
                'favicon' => '/images/logos/axel-favicon.ico',
                'apple_touch_icon' => '/images/logos/axel-apple-touch-icon.png',
            ],

            'colors' => [
                'primary' => [
                    50  => '#f0f7ff',
                    100 => '#e0efff',
                    200 => '#baddff',
                    300 => '#7cc0ff',
                    400 => '#36a3ff',
                    500 => '#0c87f2',  // Main brand color
                    600 => '#006bd6',
                    700 => '#0054ac',
                    800 => '#07458a',
                    900 => '#0c3a71',
                    950 => '#08264c',
                ],
                'secondary' => [
                    50  => '#f8fafc',
                    100 => '#f1f5f9',
                    200 => '#e2e8f0',
                    300 => '#cbd5e1',
                    400 => '#94a3b8',
                    500 => '#64748b',
                    600 => '#475569',
                    700 => '#334155',
                    800 => '#1e293b',
                    900 => '#0f172a',
                    950 => '#020617',
                ],
                'accent' => [
                    50  => '#fdf4ff',
                    100 => '#fae8ff',
                    200 => '#f5d0fe',
                    300 => '#f0abfc',
                    400 => '#e879f9',
                    500 => '#d946ef',
                    600 => '#c026d3',
                    700 => '#a21caf',
                    800 => '#86198f',
                    900 => '#701a75',
                    950 => '#4a044e',
                ],
                'success' => [
                    50  => '#f0fdf4',
                    100 => '#dcfce7',
                    200 => '#bbf7d0',
                    300 => '#86efac',
                    400 => '#4ade80',
                    500 => '#22c55e',
                    600 => '#16a34a',
                    700 => '#15803d',
                    800 => '#166534',
                    900 => '#14532d',
                ],
                'warning' => [
                    50  => '#fffbeb',
                    100 => '#fef3c7',
                    200 => '#fde68a',
                    300 => '#fcd34d',
                    400 => '#fbbf24',
                    500 => '#f59e0b',
                    600 => '#d97706',
                    700 => '#b45309',
                    800 => '#92400e',
                    900 => '#78350f',
                ],
                'destructive' => [
                    50  => '#fef2f2',
                    100 => '#fee2e2',
                    200 => '#fecaca',
                    300 => '#fca5a5',
                    400 => '#f87171',
                    500 => '#ef4444',
                    600 => '#dc2626',
                    700 => '#b91c1c',
                    800 => '#991b1b',
                    900 => '#7f1d1d',
                ],
            ],

            'typography' => [
                'font_family' => 'Inter, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
                'font_family_mono' => 'JetBrains Mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
                'font_family_display' => 'Cal Sans, Inter, system-ui, sans-serif',
            ],

            'spacing' => [
                'base_unit' => 4, // 4px base unit
            ],

            'border_radius' => [
                'none' => '0',
                'sm' => '0.25rem',
                'md' => '0.375rem',
                'lg' => '0.5rem',
                'xl' => '0.75rem',
                '2xl' => '1rem',
                '3xl' => '1.5rem',
                'full' => '9999px',
            ],

            'shadows' => [
                'xs' => '0 1px 2px 0 rgb(0 0 0 / 0.05)',
                'sm' => '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
                'md' => '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
                'lg' => '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
                'xl' => '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
                '2xl' => '0 25px 50px -12px rgb(0 0 0 / 0.25)',
                'inner' => 'inset 0 2px 4px 0 rgb(0 0 0 / 0.05)',
            ],

            'breakpoints' => [
                'xs' => '480px',
                'sm' => '640px',
                'md' => '768px',
                'lg' => '1024px',
                'xl' => '1280px',
                '2xl' => '1536px',
                '3xl' => '1920px',
            ],
        ],

        /*
         * Apexos - Platform/OS Brand
         */
        'apexos' => [
            'name' => 'Apexos',
            'tagline' => 'The operating system for learning',
            'description' => 'Core platform services powering Axel LMS',
            
            'logos' => [
                'primary' => [
                    'light' => '/images/logos/apexos-icon-primary.png',
                    'dark' => '/images/logos/apexos-icon-primary.png',
                    'width' => 140,
                    'height' => 35,
                ],
                'icon' => [
                    'light' => '/images/logos/apexos-icon-primary.png',
                    'dark' => '/images/logos/apexos-icon-primary.png',
                    'width' => 32,
                    'height' => 32,
                ],
                'secondary' => [
                    'light' => '/images/logos/apexos-icon-secondary.png',
                    'dark' => '/images/logos/apexos-icon-secondary.png',
                    'width' => 140,
                    'height' => 35,
                ],
                'accent' => [
                    'light' => '/images/logos/apexos-icon-accent.png',
                    'dark' => '/images/logos/apexos-icon-accent.png',
                    'width' => 140,
                    'height' => 35,
                ],
                'favicon' => '/images/logos/apexos-favicon.ico',
            ],

            'colors' => [
                'primary' => [
                    50  => '#faf5ff',
                    100 => '#f3e8ff',
                    200 => '#e9d5ff',
                    300 => '#d8b4fe',
                    400 => '#c084fc',
                    500 => '#a855f7',  // Apexos purple
                    600 => '#9333ea',
                    700 => '#7e22ce',
                    800 => '#6b21a8',
                    900 => '#581c87',
                    950 => '#3b0764',
                ],
            ],
        ],

        /*
         * Apex Academy - Certifications Brand
         */
        'apex_academy' => [
            'name' => 'Apex Academy',
            'tagline' => 'Verified skills, recognized globally',
            'description' => 'Professional certifications and credentials platform',
            
            'logos' => [
                'primary' => [
                    'light' => '/images/logos/apex-academy-certifications.png',
                    'dark' => '/images/logos/apex-academy-certifications.png',
                    'width' => 180,
                    'height' => 45,
                ],
                'icon' => [
                    'light' => '/images/logos/apex-academy-icon.png',
                    'dark' => '/images/logos/apex-academy-icon.png',
                    'width' => 32,
                    'height' => 32,
                ],
                'badge' => [
                    'light' => '/images/logos/apex-academy-badge.png',
                    'dark' => '/images/logos/apex-academy-badge.png',
                    'width' => 64,
                    'height' => 64,
                ],
                'favicon' => '/images/logos/apex-academy-favicon.ico',
            ],

            'colors' => [
                'primary' => [
                    50  => '#fffbeb',
                    100 => '#fef3c7',
                    200 => '#fde68a',
                    300 => '#fcd34d',
                    400 => '#fbbf24',
                    500 => '#f59e0b',  // Apex Academy gold/amber
                    600 => '#d97706',
                    700 => '#b45309',
                    800 => '#92400e',
                    900 => '#78350f',
                    950 => '#451a03',
                ],
            ],
        ],
    ],

    /*
     * Default brand to use
     */
    'default' => 'axel',

    /*
     * Brand usage contexts
     */
    'contexts' => [
        'marketing' => 'axel',
        'platform' => 'apexos',
        'certifications' => 'apex_academy',
        'admin' => 'apexos',
        'developer' => 'apexos',
    ],
];