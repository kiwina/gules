# https://developers.google.com/jules/api/reference/rest/v1alpha/sessions

  * Resource: Session  
    * JSON representation
  * SourceContext
    * JSON representation
  * GitHubRepoContext
    * JSON representation
  * AutomationMode
  * State
  * SessionOutput
    * JSON representation
  * PullRequest
    * JSON representation
  * Methods



## Resource: Session

A session is a contiguous amount of work within the same context.

JSON representation  
---  
      
    
    {
      "name": string,
      "id": string,
      "prompt": string,
      "sourceContext": {
        object ([SourceContext](/jules/api/reference/rest/v1alpha/sessions#SourceContext))
      },
      "title": string,
      "requirePlanApproval": boolean,
      "automationMode": enum ([AutomationMode](/jules/api/reference/rest/v1alpha/sessions#AutomationMode)),
      "createTime": string,
      "updateTime": string,
      "state": enum ([State](/jules/api/reference/rest/v1alpha/sessions#State)),
      "url": string,
      "outputs": [
        {
          object ([SessionOutput](/jules/api/reference/rest/v1alpha/sessions#SessionOutput))
        }
      ]
    }  
  
Fields  
---  
`name` |  `string` Output only. Identifier. The full resource name (e.g., "sessions/{session}").  
`id` |  `string` Output only. The id of the session. This is the same as the "{session}" part of the resource name (e.g., "sessions/{session}").  
`prompt` |  `string` Required. The prompt to start the session with.  
`sourceContext` |  `object (`[SourceContext](/jules/api/reference/rest/v1alpha/sessions#SourceContext)`)` Required. The source to use in this session, with additional context.  
`title` |  `string` Optional. If not provided, the system will generate one.  
`requirePlanApproval` |  `boolean` Optional. Input only. If true, plans the agent generates will require explicit plan approval before the agent starts working. If not set, plans will be auto-approved.  
`automationMode` |  `enum (`[AutomationMode](/jules/api/reference/rest/v1alpha/sessions#AutomationMode)`)` Optional. Input only. The automation mode of the session. If not set, the default automation mode will be used.  
`createTime` |  `string (`[Timestamp](https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp)` format)` Output only. The time the session was created.Uses RFC 3339, where generated output will always be Z-normalized and use 0, 3, 6 or 9 fractional digits. Offsets other than "Z" are also accepted. Examples: `"2014-10-02T15:01:23Z"`, `"2014-10-02T15:01:23.045123456Z"` or `"2014-10-02T15:01:23+05:30"`.  
`updateTime` |  `string (`[Timestamp](https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp)` format)` Output only. The time the session was last updated.Uses RFC 3339, where generated output will always be Z-normalized and use 0, 3, 6 or 9 fractional digits. Offsets other than "Z" are also accepted. Examples: `"2014-10-02T15:01:23Z"`, `"2014-10-02T15:01:23.045123456Z"` or `"2014-10-02T15:01:23+05:30"`.  
`state` |  `enum (`[State](/jules/api/reference/rest/v1alpha/sessions#State)`)` Output only. The state of the session.  
`url` |  `string` Output only. The URL of the session to view the session in the Jules web app.  
`outputs[]` |  `object (`[SessionOutput](/jules/api/reference/rest/v1alpha/sessions#SessionOutput)`)` Output only. The outputs of the session, if any.  
  
## SourceContext

Context for how to use a source in a session.

JSON representation  
---  
      
    
    {
      "source": string,
    
      // Union field context can be only one of the following:
      "githubRepoContext": {
        object ([GitHubRepoContext](/jules/api/reference/rest/v1alpha/sessions#GitHubRepoContext))
      }
      // End of list of possible types for union field context.
    }  
  
Fields  
---  
`source` |  `string` Required. The name of the source this context is for. To get the list of sources, use the ListSources API. Format: sources/{source}  
Union field `context`. The context for how to use the source in a session. `context` can be only one of the following:  
`githubRepoContext` |  `object (`[GitHubRepoContext](/jules/api/reference/rest/v1alpha/sessions#GitHubRepoContext)`)` Context to use a GitHubRepo in a session.  
  
## GitHubRepoContext

Context to use a GitHubRepo in a session.

JSON representation  
---  
      
    
    {
      "startingBranch": string
    }  
  
Fields  
---  
`startingBranch` |  `string` Required. The name of the branch to start the session from.  
  
## AutomationMode

The automation mode of the session.

Enums  
---  
`AUTOMATION_MODE_UNSPECIFIED` | The automation mode is unspecified. Default to no automation.  
`AUTO_CREATE_PR` | Whenever a final code patch is generated in the session, automatically create a branch and a pull request for it, if applicable.  
  
## State

State of a session.

Enums  
---  
`STATE_UNSPECIFIED` | The state is unspecified.  
`QUEUED` | The session is queued.  
`PLANNING` | The agent is planning.  
`AWAITING_PLAN_APPROVAL` | The agent is waiting for plan approval.  
`AWAITING_USER_FEEDBACK` | The agent is waiting for user feedback.  
`IN_PROGRESS` | The session is in progress.  
`PAUSED` | The session is paused.  
`FAILED` | The session has failed.  
`COMPLETED` | The session has completed.  
  
## SessionOutput

An output of a session.

JSON representation  
---  
      
    
    {
    
      // Union field output can be only one of the following:
      "pullRequest": {
        object ([PullRequest](/jules/api/reference/rest/v1alpha/sessions#PullRequest))
      }
      // End of list of possible types for union field output.
    }  
  
Fields  
---  
Union field `output`. An output of the session. `output` can be only one of the following:  
`pullRequest` |  `object (`[PullRequest](/jules/api/reference/rest/v1alpha/sessions#PullRequest)`)` A pull request created by the session, if applicable.  
  
## PullRequest

A pull request.

JSON representation  
---  
      
    
    {
      "url": string,
      "title": string,
      "description": string
    }  
  
Fields  
---  
`url` |  `string` The URL of the pull request.  
`title` |  `string` The title of the pull request.  
`description` |  `string` The description of the pull request.  
  
## Methods  
  
---  
  
### `[approvePlan](/jules/api/reference/rest/v1alpha/sessions/approvePlan)`

|  Approves a plan in a session.  
  
### `[create](/jules/api/reference/rest/v1alpha/sessions/create)`

|  Creates a new session.  
  
### `[get](/jules/api/reference/rest/v1alpha/sessions/get)`

|  Gets a single session.  
  
### `[list](/jules/api/reference/rest/v1alpha/sessions/list)`

|  Lists all sessions.  
  
### `[sendMessage](/jules/api/reference/rest/v1alpha/sessions/sendMessage)`

|  Sends a message from the user to a session.
