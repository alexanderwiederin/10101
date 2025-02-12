import 'package:flutter/material.dart';

/// Show a snackbar with the given message
///
/// Extracted as a separate function to ensure consistent style of error messages
void showSnackBar(BuildContext context, String message) {
  final snackBar = SnackBar(content: Text(message));
  ScaffoldMessenger.of(context).showSnackBar(snackBar);
}
