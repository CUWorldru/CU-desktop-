# CU Desktop — Experience and UX

The detailed desktop layout, interaction model, breakpoints, technical boundaries, implementation sequence, and acceptance criteria are defined in [DESKTOP-UI-ARCHITECTURE.md](./DESKTOP-UI-ARCHITECTURE.md). That document is the implementation reference for the native desktop experience.

## Product structure

CU Desktop has two modes inside one application, one account, and one shared state:
- **CU mode:** familiar CU capabilities presented through desktop-native navigation and multi-pane layouts.
- **Work mode:** a desktop workspace designed for wide screens and sustained multitasking.

Switching modes must not log the user out, fork data, or create a second identity. Desktop presentation is separate; CU business logic, permissions, and backend data remain shared.

## Work mode principles

- Keep conversations and working material visible together.
- Support multiple work areas without duplicating the underlying data model.
- Allow a chat to sit beside a route, article, document, or trip context.
- Make contextual AI aware only of the content the user is authorized to access and has chosen to provide.
- Preserve predictable navigation, keyboard accessibility, and clear focus behavior.
- Persist only local workspace/layout preferences locally; authoritative business data remains in CU's existing backend.

## Desktop frame

The standard layout consists of:
- a compact app rail for global destinations and CU/Work mode switching;
- a context-sensitive navigation pane;
- a flexible main workspace;
- an optional, resizable inspector pane.

At narrower desktop window widths, navigation and inspector regions collapse into drawers rather than forcing a mobile layout into a desktop window.

## Windows shell behavior

- Closing the primary window hides CU to the system tray by default.
- The tray menu provides Open, status/availability where supported, and Quit.
- Notifications should open the relevant CU destination rather than a generic home screen.
- Startup behavior must be visible and user-controllable.
- Permission prompts for microphone, camera, and files must be explicit and understandable.

## UX validation

Test normal window close, tray restore, true Quit, notification navigation, multiple workspaces/panes, keyboard shortcuts, high-DPI scaling, screen resizing, accessibility, and module-specific desktop layouts.
