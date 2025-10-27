package com.example.studentsapi

import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.POST

data class EmailRequest(
    val email: String,
    val subject: String,
    val message: String // Changed from 'body' to 'message' to match BFF API
)

data class EmailResponse(
    val request_id: String,
    val status: String
)

interface ApiService {
    @POST("notify/email")
    suspend fun sendEmailNotification(@Body request: EmailRequest): Response<EmailResponse>
}
