# https://developers.google.com/jules/api/reference/rest/v1alpha/sessions/sendMessage

  * HTTP request  
  * Path parameters
  * Request body
    * JSON representation
  * Response body



Sends a message from the user to a session.

### HTTP request

`POST https://jules.googleapis.com/v1alpha/{session=sessions/*}:sendMessage`

The URL uses [gRPC Transcoding](https://google.aip.dev/127) syntax.

### Path parameters

Parameters  
---  
`session` |  `string` Required. The resource name of the session to post the message to. Format: sessions/{session} It takes the form `sessions/{session}`.  
  
### Request body

The request body contains data with the following structure:

JSON representation  
---  
      
    
    {
      "prompt": string
    }  
  
Fields  
---  
`prompt` |  `string` Required. The user prompt to send to the session.  
  
### Response body

If successful, the response body is empty.
