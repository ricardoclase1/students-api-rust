package com.example.studentsapi

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.launch

class EmailViewModel : ViewModel() {

    private val apiService = RetrofitClient.apiService

    fun sendEmailNotification(
        email: String,
        subject: String,
        message: String, // Changed parameter name from 'body' to 'message'
        onResult: (Boolean, String) -> Unit
    ) {
        viewModelScope.launch {
            try {
                val request = EmailRequest(email, subject, message)
                val response = apiService.sendEmailNotification(request)

                if (response.isSuccessful) {
                    val responseBody = response.body()
                    val successMessage = responseBody?.status ?: "Email notification sent successfully!"
                    onResult(true, successMessage)
                } else {
                    onResult(false, "Failed to send notification: ${response.message()}")
                }
            } catch (e: Exception) {
                onResult(false, "Error: ${e.localizedMessage ?: "Unknown error"}")
            }
        }
    }
}
