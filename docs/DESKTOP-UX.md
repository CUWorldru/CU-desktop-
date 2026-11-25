# CU Desktop — Experience and UX

## Product structure
CU Desktop has two modes inside one application, one account, and one shared state:
- **CU mode:** familiar CU navigation and all supported existing CU modules.
- **Work mode:** a desktop workspace designed for wide screens and sustained multitasking.

Switching modes must not log the user out, fork data, or create a second identity.

## Work mode principles
- Keep conversations and working material visible together.
- Support multiple work areas without duplicating the underlying data model.
- Allow a chat to sit beside a route, article, document, or trip context.
- Make contextual AI aware only of the content the user is authorized to access and has chosen to provide.
- Preserve predictable navigation, keyboard accessibility, and clear focus behavior.

## Windows shell behavior
- Closing the primary window hides CU to the system tray by default.
- The tray menu provides Open, status/availability where supported, and Quit.
- Notifications should open the relevant CU destination rather than a generic home screen.
- Startup behavior must be visible and user-controllable.
- Permission prompts for microphone, camera, and files must be explicit and understandable.

## UX validation
Test normal window close, tray restore, true Quit, notification navigation, multiple windows/workspaces, keyboard shortcuts, high-DPI scaling, screen resizing, and accessibility.