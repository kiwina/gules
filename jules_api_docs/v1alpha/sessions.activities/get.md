# https://developers.google.com/jules/api/reference/rest/v1alpha/sessions.activities/get

  * HTTP request  
  * Path parameters
  * Request body
  * Response body



Gets a single activity.

### HTTP request

`GET https://jules.googleapis.com/v1alpha/{name=sessions/*/activities/*}`

The URL uses [gRPC Transcoding](https://google.aip.dev/127) syntax.

### Path parameters

Parameters  
---  
`name` |  `string` Required. The resource name of the activity to retrieve. Format: sessions/{session}/activities/{activity} It takes the form `sessions/{session}/activities/{activities}`.  
  
### Request body

The request body must be empty.

### Response body

If successful, the response body contains an instance of `[Activity](/jules/api/reference/rest/v1alpha/sessions.activities#Activity)`.
