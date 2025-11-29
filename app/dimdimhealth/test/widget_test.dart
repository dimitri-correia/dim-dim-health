// Basic Flutter test for DimDim Health app.
//
// This test verifies that the app can be initialized successfully.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';

import 'package:dimdimhealth/main.dart';
import 'package:dimdimhealth/services/auth_provider.dart';

void main() {
  testWidgets('MyApp initializes successfully', (WidgetTester tester) async {
    // Build the app and verify it renders without error
    await tester.pumpWidget(const MyApp());

    // Verify the app displays the loading screen initially
    expect(find.byType(MaterialApp), findsOneWidget);
  });

  testWidgets('AuthWrapper displays splash screen while loading',
      (WidgetTester tester) async {
    await tester.pumpWidget(
      ChangeNotifierProvider(
        create: (_) => AuthProvider(),
        child: const MaterialApp(
          home: AuthWrapper(),
        ),
      ),
    );

    // The splash screen should be shown initially
    expect(find.text('DimDim Health'), findsOneWidget);
  });
}
