import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';

import 'package:dimdimhealth/main.dart';
import 'package:dimdimhealth/services/auth_provider.dart';

void main() {
  testWidgets('App starts with splash screen and then shows login screen',
      (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const MyApp());

    // Verify that splash screen is shown initially
    expect(find.text('DimDim Health'), findsOneWidget);
    expect(find.text('Welcome to DimDim Health App'), findsOneWidget);

    // Wait for splash screen to complete
    await tester.pumpAndSettle(const Duration(seconds: 4));

    // Verify that login screen is shown after splash
    expect(find.text('Welcome Back'), findsOneWidget);
    expect(find.text('Email'), findsOneWidget);
    expect(find.text('Password'), findsOneWidget);
    expect(find.widgetWithText(ElevatedButton, 'Login'), findsOneWidget);
  });

  testWidgets('Login screen has register link', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const MyApp());

    // Wait for splash screen to complete
    await tester.pumpAndSettle(const Duration(seconds: 4));

    // Verify register link exists
    expect(find.text("Don't have an account? "), findsOneWidget);
    expect(find.text('Register'), findsOneWidget);
  });

  testWidgets('Login screen has forgot password link',
      (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const MyApp());

    // Wait for splash screen to complete
    await tester.pumpAndSettle(const Duration(seconds: 4));

    // Verify forgot password link exists
    expect(find.text('Forgot Password?'), findsOneWidget);
  });

  testWidgets('AuthProvider is available in widget tree',
      (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());

    // Find the provider in the widget tree
    final context = tester.element(find.byType(MyApp));
    final authProvider = Provider.of<AuthProvider>(context, listen: false);

    // Verify provider exists and user is not authenticated initially
    expect(authProvider, isNotNull);
    expect(authProvider.isAuthenticated, isFalse);
  });
}
