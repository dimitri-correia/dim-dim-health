# DimDim Health - Authentication Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         APP LAUNCH                                      │
└─────────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
                    ┌─────────────────────────┐
                    │   Splash Screen         │
                    │   (3 seconds)           │
                    └─────────────────────────┘
                                  │
                                  ▼
                    ┌─────────────────────────┐
                    │  Check Auth Status      │
                    │  (loadSavedAuth)        │
                    └─────────────────────────┘
                                  │
                ┌─────────────────┴─────────────────┐
                │                                   │
         NOT AUTHENTICATED                   AUTHENTICATED
                │                                   │
                ▼                                   ▼
    ┌───────────────────────┐           ┌───────────────────────┐
    │   Login Screen        │           │   Home Screen         │
    │   (/login)            │           │   (/home)             │
    └───────────────────────┘           └───────────────────────┘
                │                                   │
                │                                   │
    ┌───────────┼────────────┐                     │
    │           │            │                     │
    │           │            │                     │
    ▼           ▼            ▼                     ▼
┌────────┐  ┌──────┐  ┌────────────┐       ┌──────────┐
│Register│  │Forgot│  │   Login    │       │  Logout  │
│        │  │Pass  │  │   Submit   │       │          │
└────────┘  └──────┘  └────────────┘       └──────────┘
    │           │            │                     │
    │           │            │                     │
    └───────────┴────────────┼─────────────────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │   API Service   │
                    │   (Backend)     │
                    └─────────────────┘
```

## Navigation Guards

### AuthGuard (Protects /home)
```
User tries to access /home
         │
         ▼
    Authenticated?
    ┌────┴────┐
  YES         NO
    │          │
    ▼          ▼
  Allow    Redirect to /login
```

### GuestGuard (Protects /login, /register, /forgot-password)
```
User tries to access /login, /register, /forgot-password
         │
         ▼
    Authenticated?
    ┌────┴────┐
  YES         NO
    │          │
    ▼          ▼
Redirect     Allow
to /home
```

## Screen Flow Details

### Login Flow
```
Login Screen
    ↓
Enter email & password
    ↓
Validate form
    ↓
Call AuthProvider.login()
    ↓
API: POST /api/users/login
    ↓
┌───┴───┐
│SUCCESS│ERROR
↓       ↓
Save    Show
tokens  error
↓
Navigate to /home
```

### Register Flow
```
Register Screen
    ↓
Enter username, email, password, confirm password
    ↓
Validate form (including password match)
    ↓
Call AuthProvider.register()
    ↓
API: POST /api/users
    ↓
┌───┴───┐
│SUCCESS│ERROR
↓       ↓
Save    Show
tokens  error
↓
Navigate to /home
```

### Forgot Password Flow
```
Forgot Password Screen
    ↓
Enter email
    ↓
Validate form
    ↓
Call AuthProvider.forgotPassword()
    ↓
API: POST /api/auth/forgot-password
    ↓
┌───┴───┐
│SUCCESS│ERROR
↓       ↓
Show    Show
success error
message message
↓
Back to login option
```

## State Management

```
┌───────────────────────────────────────────────┐
│           AuthProvider                        │
│  (ChangeNotifier)                             │
├───────────────────────────────────────────────┤
│  State:                                       │
│  - User? _user                                │
│  - String? _accessToken                       │
│  - String? _refreshToken                      │
│  - bool _isLoading                            │
│  - String? _error                             │
│                                               │
│  Getters:                                     │
│  - User? get user                             │
│  - bool get isAuthenticated                   │
│  - bool get isLoading                         │
│  - String? get error                          │
│                                               │
│  Methods:                                     │
│  - loadSavedAuth()                            │
│  - register(username, email, password)        │
│  - login(email, password)                     │
│  - forgotPassword(email)                      │
│  - logout()                                   │
└───────────────────────────────────────────────┘
         │                        │
         │ notifyListeners()      │
         ▼                        ▼
┌─────────────────┐    ┌─────────────────┐
│  UI Screens     │    │  Route Guards   │
│  (Consumers)    │    │  (Consumers)    │
└─────────────────┘    └─────────────────┘
```

## Security Layers

```
1. Form Validation
   ↓
2. API Request (HTTPS)
   ↓
3. Backend Validation
   ↓
4. JWT Token Generation
   ↓
5. Secure Storage (FlutterSecureStorage)
   ↓
6. Route Guards (AuthGuard/GuestGuard)
```

## File Structure

```
lib/
├── main.dart                          # App entry point, routing, guards
├── models/
│   ├── user.dart                      # User data models
│   └── user.g.dart                    # Generated JSON serialization
├── screens/
│   ├── splash_screen.dart             # Initial loading screen
│   ├── login_screen.dart              # Login UI
│   ├── register_screen.dart           # Registration UI
│   ├── forgot_password_screen.dart    # Password reset UI
│   └── home_screen.dart               # Main authenticated screen
├── services/
│   ├── api_service.dart               # API client
│   └── auth_provider.dart             # State management
└── utils/
    └── app_config.dart                # App configuration

test/
└── widget_test.dart                   # Widget tests
```

## API Endpoints

```
┌────────────────────────────────────────────────────┐
│  POST /api/users                                   │
│  Body: { user: { username, email, password } }    │
│  Response: { user, access_token, refresh_token }  │
├────────────────────────────────────────────────────┤
│  POST /api/users/login                             │
│  Body: { user: { email, password } }              │
│  Response: { user, access_token, refresh_token }  │
├────────────────────────────────────────────────────┤
│  POST /api/auth/forgot-password                    │
│  Body: { email }                                   │
│  Response: { message }                             │
└────────────────────────────────────────────────────┘
```
