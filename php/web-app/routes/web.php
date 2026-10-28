<?php

use Illuminate\Support\Facades\Route;
use Inertia\Inertia;

// Public routes
Route::get('/', function () {
    return Inertia::render('HomePage');
})->name('home');

Route::get('/features', function () {
    return Inertia::render('FeaturesPage');
})->name('features');

Route::get('/pricing', function () {
    return Inertia::render('PricingPage');
})->name('pricing');

Route::get('/about', function () {
    return Inertia::render('AboutPage');
})->name('about');

Route::get('/login', function () {
    return Inertia::render('auth/LoginPage');
})->name('login')->middleware('guest');

Route::get('/register', function () {
    return Inertia::render('auth/RegisterPage');
})->name('register')->middleware('guest');

Route::get('/forgot-password', function () {
    return Inertia::render('auth/ForgotPasswordPage');
})->name('password.request')->middleware('guest');

Route::get('/reset-password/{token}', function ($token) {
    return Inertia::render('auth/ResetPasswordPage', ['token' => $token]);
})->name('password.reset')->middleware('guest');

Route::get('/verify-email/{token}', function ($token) {
    return Inertia::render('auth/VerifyEmailPage', ['token' => $token]);
})->name('verification.verify')->middleware('guest');

Route::get('/courses', function () {
    return Inertia::render('courses/CoursesPage');
})->name('courses');

Route::get('/courses/{course}', function () {
    return Inertia::render('courses/CourseDetailPage');
})->name('courses.show');

Route::get('/learning-paths', function () {
    return Inertia::render('learning-paths/LearningPathsPage');
})->name('learning-paths');

Route::get('/learning-paths/{learningPath}', function () {
    return Inertia::render('learning-paths/LearningPathDetailPage');
})->name('learning-paths.show');

// Health checks
Route::get('/up', function () {
    return response()->json(['status' => 'ok']);
})->name('health.liveness');

Route::get('/ready', function () {
    // Check database, cache, etc.
    return response()->json(['status' => 'ok']);
})->name('health.readiness');

// Authenticated routes
Route::middleware(['auth', 'verified'])->group(function () {
    // Dashboard
    Route::prefix('dashboard')->name('dashboard.')->group(function () {
        Route::get('/', function () {
            return Inertia::render('dashboard/DashboardPage');
        })->name('home');

        Route::get('/courses', function () {
            return Inertia::render('dashboard/MyCoursesPage');
        })->name('courses');

        Route::get('/learning-paths', function () {
            return Inertia::render('dashboard/MyLearningPathsPage');
        })->name('learning-paths');

        Route::get('/schedule', function () {
            return Inertia::render('dashboard/SchedulePage');
        })->name('schedule');

        Route::get('/achievements', function () {
            return Inertia::render('dashboard/AchievementsPage');
        })->name('achievements');

        Route::get('/certificates', function () {
            return Inertia::render('dashboard/CertificatesPage');
        })->name('certificates');

        Route::get('/notes', function () {
            return Inertia::render('dashboard/NotesPage');
        })->name('notes');

        Route::get('/bookmarks', function () {
            return Inertia::render('dashboard/BookmarksPage');
        })->name('bookmarks');

        Route::get('/settings', function () {
            return Inertia::render('dashboard/SettingsPage');
        })->name('settings');

        Route::get('/profile', function () {
            return Inertia::render('dashboard/ProfilePage');
        })->name('profile');
    });

    // Course Learning
    Route::get('/courses/{course}/learn', function () {
        return Inertia::render('courses/CourseLearnPage');
    })->name('course.learn');

    Route::get('/learning-paths/{learningPath}/learn', function () {
        return Inertia::render('learning-paths/LearningPathLearnPage');
    })->name('learning-path.learn');

    // Calendar
    Route::get('/calendar', function () {
        return Inertia::render('calendar/CalendarPage');
    })->name('calendar');

    // Community
    Route::get('/community', function () {
        return Inertia::render('community/CommunityPage');
    })->name('community');

    // Library
    Route::get('/library', function () {
        return Inertia::render('library/LibraryPage');
    })->name('library');

    // Messages
    Route::get('/messages', function () {
        return Inertia::render('messages/MessagesPage');
    })->name('messages');

    // Study Groups
    Route::get('/study-groups', function () {
        return Inertia::render('study-groups/StudyGroupsPage');
    })->name('study-groups');
});

// Admin routes
Route::middleware(['auth', 'role:admin'])->prefix('admin')->name('admin.')->group(function () {
    Route::get('/', function () {
        return Inertia::render('admin/AdminDashboardPage');
    })->name('home');

    Route::get('/users', function () {
        return Inertia::render('admin/users/UsersPage');
    })->name('users');

    Route::get('/users/{user}', function () {
        return Inertia::render('admin/users/UserDetailPage');
    })->name('users.show');

    Route::get('/courses', function () {
        return Inertia::render('admin/courses/CoursesPage');
    })->name('courses');

    Route::get('/courses/{course}', function () {
        return Inertia::render('admin/courses/CourseDetailPage');
    })->name('courses.show');

    Route::get('/courses/create', function () {
        return Inertia::render('admin/courses/CreateCoursePage');
    })->name('courses.create');

    Route::get('/enrollments', function () {
        return Inertia::render('admin/enrollments/EnrollmentsPage');
    })->name('enrollments');

    Route::get('/analytics', function () {
        return Inertia::render('admin/analytics/AnalyticsPage');
    })->name('analytics');

    Route::get('/settings', function () {
        return Inertia::render('admin/settings/SettingsPage');
    })->name('settings');

    Route::get('/roles', function () {
        return Inertia::render('admin/roles/RolesPage');
    })->name('roles');

    Route::get('/tenants', function () {
        return Inertia::render('admin/tenants/TenantsPage');
    })->name('tenants')->middleware('role:super_admin');

    Route::get('/audit-logs', function () {
        return Inertia::render('admin/audit/AuditLogsPage');
    })->name('audit-logs');

    Route::get('/system-health', function () {
        return Inertia::render('admin/system/SystemHealthPage');
    })->name('system-health');
});

// Error pages
Route::get('/403', function () {
    return Inertia::render('errors/ForbiddenPage');
})->name('forbidden');

Route::get('/404', function () {
    return Inertia::render('errors/NotFoundPage');
})->name('not-found');

Route::get('/500', function () {
    return Inertia::render('errors/ServerErrorPage');
})->name('server-error');

Route::get('/maintenance', function () {
    return Inertia::render('errors/MaintenancePage');
})->name('maintenance');

// Fallback
Route::fallback(function () {
    return Inertia::render('errors/NotFoundPage');
})->name('fallback');