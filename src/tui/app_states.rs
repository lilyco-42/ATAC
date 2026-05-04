use crate::t;
use crokey::{key, KeyCombination};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::Span;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use crate::app::app::App;
use crate::app::files::key_bindings::{CustomTextArea, TextAreaMode, KEY_BINDINGS};
use crate::app::files::theme::THEME;
use crate::models::protocol::protocol::Protocol;
use crate::tui::app_states::AppState::*;
use crate::tui::event_key_bindings::EventKeyBinding;
use crate::tui::events::AppEvent;
use crate::tui::events::AppEvent::*;
use crate::tui::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::tui::ui::views::RequestView;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum AppState {
    #[default]
    Normal,

    /* Env */

    DisplayingEnvEditor,

    EditingEnvVariable,

    /* Cookies */

    DisplayingCookies,

    #[allow(dead_code)]
    EditingCookies,

    /* Logs */

    DisplayingLogs,

    /* Collections */

    ChoosingElementToCreate,

    CreatingNewCollection,

    CreatingNewRequest,

    DeletingCollection,

    DeletingRequest,

    RenamingCollection,

    RenamingRequest,

    /* Request */

    SelectedRequest,

    EditingRequestUrl,

    EditingRequestParam,

    EditingRequestAuthBasicUsername,

    EditingRequestAuthBasicPassword,

    EditingRequestAuthBearerToken,

    EditingRequestAuthJwtSecret,

    EditingRequestAuthJwtPayload,

    EditingRequestAuthDigestUsername,

    EditingRequestAuthDigestPassword,

    EditingRequestAuthDigestDomains,

    EditingRequestAuthDigestRealm,

    EditingRequestAuthDigestNonce,

    EditingRequestAuthDigestOpaque,

    EditingRequestHeader,

    EditingRequestBodyTable,

    EditingRequestBodyFile,

    EditingRequestBodyString,

    EditingRequestMessage,

    EditingPreRequestScript,

    EditingPostRequestScript,

    EditingRequestSettings,

    ChoosingRequestExportFormat,

    DisplayingRequestExport,
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Normal => write!(f, "{}", t!("Main menu")),
            DisplayingEnvEditor => write!(f, "{}", t!("Displaying environment editor")),
            EditingEnvVariable => write!(f, "{}", t!("Editing env variable")),
            DisplayingCookies => write!(f, "{}", t!("Displaying cookies")),
            EditingCookies => write!(f, "{}", t!("Editing cookies")),
            DisplayingLogs => write!(f, "{}", t!("Displaying logs")),
            ChoosingElementToCreate => write!(f, "{}", t!("Choosing an element to create")),
            CreatingNewCollection => write!(f, "{}", t!("Creating new collection")),
            CreatingNewRequest => write!(f, "{}", t!("Creating new request")),
            DeletingCollection => write!(f, "{}", t!("Deleting collection")),
            DeletingRequest => write!(f, "{}", t!("Deleting request")),
            RenamingCollection => write!(f, "{}", t!("Renaming collection")),
            RenamingRequest => write!(f, "{}", t!("Renaming request")),
            SelectedRequest => write!(f, "{}", t!("Request menu")),
            EditingRequestUrl => write!(f, "{}", t!("Editing request URL")),
            EditingRequestParam => write!(f, "{}", t!("Editing request param")),
            EditingRequestAuthBasicUsername => write!(f, "{}", t!("Editing request auth username")),
            EditingRequestAuthBasicPassword => write!(f, "{}", t!("Editing request auth password")),
            EditingRequestAuthBearerToken => write!(f, "{}", t!("Editing request auth bearer token")),
            EditingRequestAuthJwtSecret => write!(f, "{}", t!("Editing request JWT secret")),
            EditingRequestAuthJwtPayload => write!(f, "{}", t!("Editing request JWT payload")),
            EditingRequestAuthDigestUsername => write!(f, "{}", t!("Editing request digest username")),
            EditingRequestAuthDigestPassword => write!(f, "{}", t!("Editing request digest password")),
            EditingRequestAuthDigestDomains => write!(f, "{}", t!("Editing request digest domains")),
            EditingRequestAuthDigestRealm => write!(f, "{}", t!("Editing request digest realm")),
            EditingRequestAuthDigestNonce => write!(f, "{}", t!("Editing request digest nonce")),
            EditingRequestAuthDigestOpaque => write!(f, "{}", t!("Editing request digest opaque")),
            EditingRequestHeader => write!(f, "{}", t!("Editing request header")),
            EditingRequestBodyTable => write!(f, "{}", t!("Editing request body (Form)")),
            EditingRequestBodyFile => write!(f, "{}", t!("Editing request body (File)")),
            EditingRequestBodyString => write!(f, "{}", t!("Editing request body (Text)")),
            EditingRequestMessage => write!(f, "{}", t!("Editing request message")),
            EditingPreRequestScript => write!(f, "{}", t!("Editing pre-request script")),
            EditingPostRequestScript => write!(f, "{}", t!("Editing post-request script")),
            EditingRequestSettings => write!(f, "{}", t!("Editing request settings")),
            ChoosingRequestExportFormat => write!(f, "{}", t!("Choosing request export format")),
            DisplayingRequestExport => write!(f, "{}", t!("Displaying request export")),
        }
    }
}

pub fn next_app_state(app_state: &AppState) -> AppState {
    match app_state {
        Normal => DisplayingEnvEditor,
        DisplayingEnvEditor => EditingEnvVariable,
        EditingEnvVariable => DisplayingCookies,
        DisplayingCookies => EditingCookies,
        EditingCookies => DisplayingLogs,
        DisplayingLogs => ChoosingElementToCreate,
        ChoosingElementToCreate => CreatingNewCollection,
        CreatingNewCollection => CreatingNewRequest,
        CreatingNewRequest => DeletingCollection,
        DeletingCollection => DeletingRequest,
        DeletingRequest => RenamingCollection,
        RenamingCollection => RenamingRequest,
        RenamingRequest => SelectedRequest,
        SelectedRequest => EditingRequestUrl,
        EditingRequestUrl => EditingRequestParam,
        EditingRequestParam => EditingRequestAuthBasicUsername,
        EditingRequestAuthBasicUsername => EditingRequestAuthBasicPassword,
        EditingRequestAuthBasicPassword => EditingRequestAuthBearerToken,
        EditingRequestAuthBearerToken => EditingRequestAuthJwtSecret,
        EditingRequestAuthJwtSecret => EditingRequestAuthJwtPayload,
        EditingRequestAuthJwtPayload => EditingRequestAuthDigestUsername,
        EditingRequestAuthDigestUsername => EditingRequestAuthDigestPassword,
        EditingRequestAuthDigestPassword => EditingRequestAuthDigestDomains,
        EditingRequestAuthDigestDomains => EditingRequestAuthDigestRealm,
        EditingRequestAuthDigestRealm => EditingRequestAuthDigestNonce,
        EditingRequestAuthDigestNonce => EditingRequestAuthDigestOpaque,
        EditingRequestAuthDigestOpaque => EditingRequestHeader,
        EditingRequestHeader => EditingRequestBodyTable,
        EditingRequestBodyTable => EditingRequestBodyFile,
        EditingRequestBodyFile => EditingRequestBodyString,
        EditingRequestBodyString => EditingRequestMessage,
        EditingRequestMessage => EditingPreRequestScript,
        EditingPreRequestScript => EditingPostRequestScript,
        EditingPostRequestScript => EditingRequestSettings,
        EditingRequestSettings => ChoosingRequestExportFormat,
        ChoosingRequestExportFormat => DisplayingRequestExport,
        DisplayingRequestExport => Normal
    }
}

pub fn previous_app_state(app_state: &AppState) -> AppState {
    match app_state {
        Normal => EditingRequestSettings,
        DisplayingEnvEditor => Normal,
        EditingEnvVariable => DisplayingEnvEditor,
        DisplayingCookies => EditingEnvVariable,
        EditingCookies => DisplayingCookies,
        DisplayingLogs => EditingCookies,
        ChoosingElementToCreate => DisplayingLogs,
        CreatingNewCollection => ChoosingElementToCreate,
        CreatingNewRequest => CreatingNewCollection,
        DeletingCollection => CreatingNewRequest,
        DeletingRequest => DeletingCollection,
        RenamingCollection => DeletingRequest,
        RenamingRequest => RenamingCollection,
        SelectedRequest => RenamingRequest,
        EditingRequestUrl => SelectedRequest,
        EditingRequestParam => EditingRequestUrl,
        EditingRequestAuthBasicUsername => EditingRequestParam,
        EditingRequestAuthBasicPassword => EditingRequestAuthBasicUsername,
        EditingRequestAuthBearerToken => EditingRequestAuthBasicPassword,
        EditingRequestAuthJwtSecret => EditingRequestAuthBearerToken,
        EditingRequestAuthJwtPayload => EditingRequestAuthJwtSecret,
        EditingRequestAuthDigestUsername => EditingRequestAuthJwtPayload,
        EditingRequestAuthDigestPassword => EditingRequestAuthDigestUsername,
        EditingRequestAuthDigestDomains => EditingRequestAuthDigestPassword,
        EditingRequestAuthDigestRealm => EditingRequestAuthDigestDomains,
        EditingRequestAuthDigestNonce => EditingRequestAuthDigestRealm,
        EditingRequestAuthDigestOpaque => EditingRequestAuthDigestNonce,
        EditingRequestHeader => EditingRequestAuthDigestOpaque,
        EditingRequestBodyTable => EditingRequestHeader,
        EditingRequestBodyFile => EditingRequestBodyTable,
        EditingRequestBodyString => EditingRequestBodyFile,
        EditingRequestMessage => EditingRequestBodyString,
        EditingPreRequestScript => EditingRequestMessage,
        EditingPostRequestScript => EditingPreRequestScript,
        EditingRequestSettings => EditingPostRequestScript,
        ChoosingRequestExportFormat => EditingRequestSettings,
        DisplayingRequestExport => ChoosingRequestExportFormat
    }
}

impl AppState {
    pub fn get_available_events(&self, request_view: RequestView, request_param_tab: RequestParamsTabs, protocol: Option<Protocol>, is_there_any_env: bool) -> Vec<AppEvent> {
        let key_bindings = KEY_BINDINGS.read();

        match self {
            Normal => {
                let mut base_events = vec![
                    ExitApp(EventKeyBinding::new(vec![key_bindings.main_menu.exit, key!(ctrl-c)], t!("Exit"), Some("Exit"))),

                    Documentation(EventKeyBinding::new(vec![key_bindings.generic.display_help], t!("Display help"), Some("Help"))),

                    MoveCollectionCursorUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), Some("Up"))),
                    MoveCollectionCursorDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), Some("Down"))),

                    SelectRequestOrExpandCollection(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], t!("Select"), Some("Select"))),
                    UnselectRequest(EventKeyBinding::new(vec![key_bindings.main_menu.unselect_request], t!("Unselect"), None)),
                    ExpandCollection(EventKeyBinding::new(vec![key_bindings.main_menu.expand_collection], t!("Expand"), None)),

                    CreateElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], t!("Create element"), Some("Create"))),
                    DeleteElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], t!("Delete element"), None)),
                    RenameElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.rename_element], t!("Rename element"), None)),
                    DuplicateElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], t!("Duplicate element"), None)),

                    MoveElementUp(EventKeyBinding::new(vec![key_bindings.main_menu.move_request_up], t!("Move request up"), None)),
                    MoveElementDown(EventKeyBinding::new(vec![key_bindings.main_menu.move_request_down], t!("Move request down"), None)),
                ];

                if is_there_any_env {
                    let env_events = vec![
                        NextEnvironment(EventKeyBinding::new(vec![key_bindings.main_menu.next_environment], t!("Next environment"), None)),
                        DisplayEnvEditor(EventKeyBinding::new(vec![key_bindings.main_menu.display_env_editor], t!("Environment editor"), None)),
                    ];
                    
                    base_events.extend(env_events);
                }

                let other_events = vec![
                    DisplayCookies(EventKeyBinding::new(vec![key_bindings.main_menu.display_cookies], t!("Display cookies"), None)),
                    DisplayLogs(EventKeyBinding::new(vec![key_bindings.main_menu.display_logs], t!("Display logs"), None)),
                ];
                
                base_events.extend(other_events);
                
                base_events
            },
            DisplayingEnvEditor => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit"), Some("Quit"))),
                EditEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit env variable"), None)),

                EnvVariablesMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), Some("Up"))),
                EnvVariablesMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), Some("Down"))),
                EnvVariablesMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move left"), Some("Left"))),
                EnvVariablesMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move right"), Some("Right"))),

                CreateEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], t!("Create env variable"), Some("Create variable"))),
                DeleteEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], t!("Delete env variable"), Some("Delete variable"))),
            ],
            EditingEnvVariable => [
                vec![
                    ModifyEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelModifyEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventModifyEnvVariable(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            DisplayingCookies => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit"), Some("Quit"))),

                CookiesMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), Some("Up"))),
                CookiesMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), Some("Down"))),
                CookiesMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move left"), Some("Left"))),
                CookiesMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move right"), Some("Right"))),

                DeleteCookie(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], t!("Delete cookie"), Some("Delete"))),
            ],
            EditingCookies => vec![
                Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], t!("Not implemented yet"), None))
            ],
            DisplayingLogs => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit"), Some("Quit"))),
                ScrollLogsUp(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_up], t!("Scroll logs up"), Some("Up"))),
                ScrollLogsDown(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_down], t!("Scroll logs down"), Some("Down"))),
                ScrollLogsLeft(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_left], t!("Scroll logs left"), Some("Left"))),
                ScrollLogsRight(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_right], t!("Scroll logs right"), Some("Right"))),
            ],
            ChoosingElementToCreate => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit"), Some("Quit"))),

                ChooseElementToCreateMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move selection left"), Some("Left"))),
                ChooseElementToCreateMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move selection right"), Some("Right"))),

                SelectElementToCreate(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], t!("Select element to create"), Some("Select"))),
            ],
            CreatingNewCollection => [
                vec![
                    CreateNewCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelCreateNewCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventCreateNewCollection(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            CreatingNewRequest => [
                vec![
                    CreateNewRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelCreateNewRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),

                    CreatingRequestSelectInputUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.alt_move_cursor_up], t!("Input selection up"), Some("Up"))),
                    CreatingRequestSelectInputDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.alt_move_cursor_down], t!("Input selection down"), Some("Down"))),
                    CreatingRequestInputLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Previous"), Some("Left"))),
                    CreatingRequestInputRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Next"), Some("Right"))),

                    KeyEventCreateNewRequest(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            DeletingCollection => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Cancel"), Some("Cancel"))),

                DeletingCollectionMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move selection left"), Some("Left"))),
                DeletingCollectionMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move selection right"), Some("Right"))),

                DeleteCollection(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], t!("Select choice"), Some("Select"))),
            ],
            DeletingRequest => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Cancel"), Some("Cancel"))),

                DeletingRequestMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move selection left"), Some("Left"))),
                DeletingRequestMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move selection right"), Some("Right"))),

                DeleteRequest(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], t!("Select choice"), Some("Select"))),

            ],
            RenamingCollection => [
                vec![
                    RenameCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelRenameCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventRenameCollection(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            RenamingRequest => [
                vec![
                    RenameRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelRenameRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventRenameRequest(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            SelectedRequest => {
                // Depending on the current request view, some keys may need to be deactivated
                let (params_events_allowed, result_events_allowed) = match request_view {
                    RequestView::Normal => (true, true),
                    RequestView::OnlyResult => (false, true),
                    RequestView::OnlyParams => (true, false)
                };

                let mut base_events: Vec<AppEvent> = vec![
                    ExitApp(EventKeyBinding::new(vec![key!(ctrl-c)], t!("Exit app"), None)),

                    GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit to main menu"), Some("Quit"))),
                    Documentation(EventKeyBinding::new(vec![key_bindings.generic.display_help], t!("Display help"), Some("Help"))),

                    EditUrl(EventKeyBinding::new(vec![key_bindings.request_selected.change_url], t!("Edit URL"), Some("URL"))),

                    EditSettings(EventKeyBinding::new(vec![key_bindings.request_selected.request_settings], t!("Request settings"), None)),

                    NextView(EventKeyBinding::new(vec![key_bindings.request_selected.next_view], t!("Next view"), None)),

                    SendRequest(EventKeyBinding::new(vec![key_bindings.request_selected.send_request, key_bindings.request_selected.alt_send_request], t!("Send/cancel request"), Some("Send/Cancel"))),
                ];
                
                if is_there_any_env {
                    let env_events = vec![
                        NextEnvironment(EventKeyBinding::new(vec![key_bindings.main_menu.next_environment], t!("Next environment"), None)),
                        DisplayEnvEditor(EventKeyBinding::new(vec![key_bindings.main_menu.display_env_editor], t!("Environment editor"), None)),
                    ];
                    
                    base_events.extend(env_events);
                }
                
                let other_events = vec![
                    DisplayCookies(EventKeyBinding::new(vec![key_bindings.main_menu.display_cookies], t!("Display cookies"), None)),
                    DisplayLogs(EventKeyBinding::new(vec![key_bindings.main_menu.display_logs], t!("Display logs"), None)),
                    ExportRequest(EventKeyBinding::new(vec![key_bindings.request_selected.export_request], t!("Export request"), None)),
                ];
                
                base_events.extend(other_events);

                let mut base_param_tabs_events: Vec<AppEvent> = vec![];
                let mut base_result_tabs_events: Vec<AppEvent> = vec![];

                // Param tabs
                if params_events_allowed {
                    base_param_tabs_events = vec![
                        NextParamTab(EventKeyBinding::new(vec![key_bindings.request_selected.param_next_tab], t!("Next param tab"), Some("Next tab"))),

                        ModifyRequestAuthMethod(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_auth_method], t!("Modify auth method"), None)),
                    ];

                    if let Some(protocol) = protocol {
                        let protocol_specific = match protocol {
                            Protocol::HttpRequest(_) => vec![
                                EditMethod(EventKeyBinding::new(vec![key_bindings.request_selected.change_method], t!("Change method"), Some("Method"))),
                                ModifyRequestBodyContentType(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_body_content_type], t!("Modify body content-type"), None)),
                            ],
                            Protocol::WsRequest(_) => vec![
                                ModifyRequestMessageType(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_message_type], t!("Modify message type"), None)),
                            ]
                        };

                        base_param_tabs_events.extend(protocol_specific);

                    }

                    let param_tabs_events = match request_param_tab {
                        RequestParamsTabs::QueryParams => vec![
                            EditRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit query param"), None)),

                            RequestQueryParamsMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), None)),
                            RequestQueryParamsMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), None)),
                            RequestQueryParamsMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move left"), None)),
                            RequestQueryParamsMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move right"), None)),

                            CreateRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], t!("Create query param"), None)),
                            DeleteRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], t!("Delete query param"), None)),
                            ToggleRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.toggle_element], t!("Toggle query param"), None)),
                            DuplicateRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], t!("Duplicate query param"), None)),
                        ],
                        RequestParamsTabs::Auth => vec![
                            EditRequestAuth(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit auth element"), None)),

                            RequestAuthMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), None)),
                            RequestAuthMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), None)),
                            RequestAuthMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move left"), None)),
                            RequestAuthMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move right"), None)),
                        ],
                        RequestParamsTabs::Headers => vec![
                            EditRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit header"), None)),

                            RequestHeadersMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), None)),
                            RequestHeadersMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), None)),
                            RequestHeadersMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move left"), None)),
                            RequestHeadersMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move right"), None)),

                            CreateRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], t!("Create header"), None)),
                            DeleteRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], t!("Delete header"), None)),
                            ToggleRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.toggle_element], t!("Toggle header"), None)),
                            DuplicateRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], t!("Duplicate header"), None)),
                        ],
                        RequestParamsTabs::Body => vec![
                            EditRequestBody(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit body"), None)),

                            RequestBodyTableMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), None)),
                            RequestBodyTableMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), None)),
                            RequestBodyTableMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move left"), None)),
                            RequestBodyTableMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move right"), None)),

                            CreateRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], t!("Create form element"), None)),
                            DeleteRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], t!("Delete form element"), None)),
                            ToggleRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.toggle_element], t!("Toggle form element"), None)),
                            DuplicateRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], t!("Duplicate form element"), None)),
                        ],
                        RequestParamsTabs::Message => vec![
                            EditRequestMessage(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit message"), None)),
                        ],
                        RequestParamsTabs::Scripts => vec![
                            EditRequestScript(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], t!("Edit request script"), Some("Edit"))),
                            RequestScriptMove(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), Some("Up"))),
                            RequestScriptMove(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), Some("Down"))),
                        ]
                    };

                    base_param_tabs_events.extend(param_tabs_events);
                }
                else {
                    base_events.push(
                        NextResultTab(EventKeyBinding::new(vec![key_bindings.request_selected.param_next_tab], t!("Next result tab"), Some("Next tab"))),
                    );
                }

                if result_events_allowed {
                    base_result_tabs_events = vec![
                        ScrollResultUp(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_up], t!("Scroll result up"), None)),
                        ScrollResultDown(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_down], t!("Scroll result down"), None)),
                        ScrollResultLeft(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_left], t!("Scroll result left"), None)),
                        ScrollResultRight(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_right], t!("Scroll result right"), None)),
                    
                        CopyResponsePart(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.yank_response_part], t!("Yank response part"), Some("Yank"))),
                    ];

                    if params_events_allowed {
                        base_events.push(
                            NextResultTab(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.result_next_tab], t!("Next result tab"), None)),
                        )
                    }
                }

                base_events.extend(base_param_tabs_events);
                base_events.extend(base_result_tabs_events);

                base_events
            },
            EditingRequestUrl => [
                vec![
                    ModifyRequestUrl(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestUrl(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestUrl(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestParam => [
                vec![
                    ModifyRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestQueryParam(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            EditingRequestAuthBasicUsername => [
                vec![
                    ModifyRequestAuthBasicUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthBasicUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthBasicUsername(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthBasicPassword => [
                vec![
                    ModifyRequestAuthBasicPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthBasicPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthBasicPassword(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthBearerToken => [
                vec![
                    ModifyRequestAuthBearerToken(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthBearerToken(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthBearerToken(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthJwtSecret => [
                vec![
                    ModifyRequestAuthJwtSecret(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthJwtSecret(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthJwtSecret(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthJwtPayload => [
                vec![
                    ModifyRequestAuthJwtPayload(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthJwtPayload(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthJwtPayload(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingRequestAuthDigestUsername => [
                vec![
                    ModifyRequestAuthDigestUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthDigestUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthDigestUsername(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestPassword => [
                vec![
                    ModifyRequestAuthDigestPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthDigestPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthDigestPassword(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestDomains => [
                vec![
                    ModifyRequestAuthDigestDomains(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthDigestDomains(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthDigestDomains(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestRealm => [
                vec![
                    ModifyRequestAuthDigestRealm(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthDigestRealm(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthDigestRealm(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestNonce => [
                vec![
                    ModifyRequestAuthDigestNonce(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthDigestNonce(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthDigestNonce(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestOpaque => [
                vec![
                    ModifyRequestAuthDigestOpaque(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestAuthDigestOpaque(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestAuthDigestOpaque(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestHeader => [
                vec![
                    ModifyRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestHeader(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            EditingRequestBodyTable => [
                vec![
                    ModifyRequestBodyTable(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestBodyTable(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestBodyTable(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            EditingRequestBodyFile => [
                vec![
                    ModifyRequestBodyFile(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestBodyFile(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestBodyFile(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestBodyString => [
                vec![
                    ModifyRequestBodyString(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestBodyString(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestBodyString(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingRequestMessage => [
                vec![
                    ModifyRequestMessage(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestMessage(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestMessage(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingPreRequestScript => [
                vec![
                    ModifyRequestPreRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestPreRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestPreRequestScript(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingPostRequestScript => [
                vec![
                    ModifyRequestPostRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], t!("Confirm"), Some("Confirm"))),
                    CancelEditRequestPostRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], t!("Cancel"), Some("Cancel"))),
                    KeyEventEditRequestPostRequestScript(EventKeyBinding::new(vec![], t!("Any input"), None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingRequestSettings => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Cancel"), Some("Cancel"))),

                RequestSettingsMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], t!("Move up"), Some("Up"))),
                RequestSettingsMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], t!("Move down"), Some("Down"))),
                RequestSettingsToggleSettingLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Toggle setting"), Some("Toggle left"))),
                RequestSettingsToggleSettingRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Toggle setting"), Some("Toggle right"))),

                ModifyRequestSettings(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], t!("Confirm"), Some("Confirm"))),
            ],
            ChoosingRequestExportFormat => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit"), Some("Quit"))),

                RequestExportFormatMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], t!("Move selection left"), Some("Left"))),
                RequestExportFormatMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], t!("Move selection right"), Some("Right"))),

                SelectRequestExportFormat(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], t!("Select export format"), Some("Select"))),
            ],
            DisplayingRequestExport => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], t!("Quit"), Some("Quit"))),

                ScrollRequestExportUp(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_up], t!("Scroll request export up"), None)),
                ScrollRequestExportDown(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_down], t!("Scroll request export down"), None)),
                ScrollRequestExportLeft(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_left], t!("Scroll request export left"), None)),
                ScrollRequestExportRight(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_right], t!("Scroll request export right"), None)),

                CopyRequestExport(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.yank_response_part], t!("Yank request export"), Some("Yank"))),
            ]
        }
    }
}

fn generate_text_input_documentation(text_input_mode: TextAreaMode, single_line: bool, insert_mode_only: bool) -> Vec<AppEvent> {
    let mut initial = Vec::new();

    match text_input_mode {
        TextAreaMode::Vim => {
            if !single_line {
                initial.push(Documentation(EventKeyBinding::new(vec![key!(ctrl-e)], t!("System editor"), None)));
            }

            if !insert_mode_only {
                initial.extend(vec![
                    Documentation(EventKeyBinding::new(vec![key!(esc)], t!("Normal mode"), Some("Esc"))),
                    Documentation(EventKeyBinding::new(vec![key!(i)], t!("Enter insert mode"), None)),
                    Documentation(EventKeyBinding::new(vec![key!(v)], t!("Enter visual mode"), None)),
                    Documentation(EventKeyBinding::new(vec![key!('/')], t!("Start search"), Some("Search"))),
                ]);
            }

        initial.extend(vec![
            Documentation(EventKeyBinding::new(vec![key!(y)], t!("Copy selection"), None)),
            Documentation(EventKeyBinding::new(vec![key!(y), key!(y)], t!("Copy line"), None)),
            Documentation(EventKeyBinding::new(vec![key!(p)], t!("Paste"), None)),
            Documentation(EventKeyBinding::new(vec![key!(u)], t!("Undo"), Some("Undo"))),
            Documentation(EventKeyBinding::new(vec![key!(ctrl-r)], t!("Redo"), Some("Redo"))),
            Documentation(EventKeyBinding::new(vec![key!(w)], t!("Move to next word"), None)),
            Documentation(EventKeyBinding::new(vec![key!(e)], t!("Move to end of word"), None)),
            Documentation(EventKeyBinding::new(vec![key!(b)], t!("Move to previous word"), None)),
            Documentation(EventKeyBinding::new(vec![key!(0)], t!("Move to start of line"), None)),
            Documentation(EventKeyBinding::new(vec![key!('$')], t!("Move to end of line"), None)),
            Documentation(EventKeyBinding::new(vec![key!(g), key!(g)], t!("Move to first line"), None)),
            Documentation(EventKeyBinding::new(vec![key!(G)], t!("Move to last line"), None)),
            Documentation(EventKeyBinding::new(vec![key!(a)], t!("Append after cursor"), None)),
            Documentation(EventKeyBinding::new(vec![key!(o)], t!("Insert line below"), None)),
            Documentation(EventKeyBinding::new(vec![key!(O)], t!("Insert line above"), None)),
            Documentation(EventKeyBinding::new(vec![key!(enter)], t!("Insert line break"), None)),
            Documentation(EventKeyBinding::new(vec![key!(x)], t!("Delete char"), None)),
            Documentation(EventKeyBinding::new(vec![key!(d), key!(d)], t!("Delete line"), None)),
            Documentation(EventKeyBinding::new(vec![key!(D)], t!("Delete to end of line"), None)),
            Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], t!("Many other vim commands..."), None)),
          ]);
        },
        TextAreaMode::Emacs => {
            if !single_line {
                initial.push(Documentation(EventKeyBinding::new(vec![key!(alt-e)], t!("System editor"), None)));
            }

            initial.extend(vec![
                Documentation(EventKeyBinding::new(vec![key!(ctrl-u)], t!("Undo"), Some("Undo"))),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-r)], t!("Redo"), Some("Redo"))),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-y)], t!("Paste"), None)),
                Documentation(EventKeyBinding::new(vec![key!(backspace)], t!("Remove char from search"), None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-k)], t!("Delete to end of line"), None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-o)], t!("Insert line break above"), None)),
                Documentation(EventKeyBinding::new(vec![key!(enter)], t!("Insert line break"), None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-j)], t!("Insert line break"), None)),
                Documentation(EventKeyBinding::new(vec![key!(backspace)], t!("Delete previous char"), None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-h)], t!("Delete previous char"), None)),
                Documentation(EventKeyBinding::new(vec![key!(backspace)], t!("Delete next char"), None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-d)], t!("Delete next char"), None)),
                Documentation(EventKeyBinding::new(vec![key!(alt-d)], t!("Delete next word"), None)),
                Documentation(EventKeyBinding::new(vec![key!(alt-backspace)], t!("Delete previous word"), None)),
                Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], t!("Many other emacs shortcuts..."), None)),
            ]);

            if !single_line {
                initial.extend(vec![
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-s)], t!("Start search"), Some("Search"))),
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-s)], t!("Find next match"), None)),
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-r)], t!("Find previous match"), None)),
                    Documentation(EventKeyBinding::new(vec![key!(enter)], t!("Select current search result"), None)),
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-g)], t!("Stop search"), None)),
                ]);
            }
        }
        _ => {
            let custom_text_area_bindings = match text_input_mode {
                TextAreaMode::Default => CustomTextArea::default(),
                TextAreaMode::Custom(custom_text_area_bindings) => custom_text_area_bindings,
                _ => unreachable!()
            };

            initial.extend(vec![
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.delete_backward], t!("Delete char backward"), None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.delete_forward], t!("Delete char forward"), None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_left], t!("Move cursor left"), None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_right], t!("Move cursor right"), None)),
            ]);

            if !single_line {
                initial.extend(vec![
                    Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_up], t!("Move cursor up"), None)),
                    Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_down], t!("Move cursor down"), None)),
                ]);
            }

            initial.extend(vec![
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_line_start], t!("Move cursor line start"), Some("Home"))),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_line_end], t!("Move cursor line end"), Some("End"))),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.skip_word_left], t!("Skip word left"), None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.skip_word_right], t!("Skip word right"), None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.undo], t!("Undo"), Some("Undo"))),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.redo], t!("Redo"), None)),
            ]);

            if !insert_mode_only {
                initial.push(Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.search], t!("Search"), Some("Search"))));
            }
        }
    }

    initial
}

pub fn event_available_keys_to_spans(events: &Vec<AppEvent>, fg_color: Color, bg_color: Color, short_only: bool) -> Vec<Vec<Span<'_>>> {
    let mut spans: Vec<Vec<Span>> = vec![];

    for event in events.iter() {
        let is_documentation = match event {
            Documentation(_) => true,
            _ => false
        };

        let event_key_bindings = event.get_event_key_bindings();

        if let Some(key_spans) = event_key_bindings.to_spans(fg_color, bg_color, short_only, is_documentation) {
            spans.push(key_spans);
        }
    }

    spans.last_mut().unwrap().pop();

    return spans;
}

lazy_static! {
    pub static ref AVAILABLE_EVENTS: RwLock<Vec<AppEvent>> = RwLock::new(vec![]);
    pub static ref EMPTY_KEY: KeyCombination = KeyCombination::new(KeyCode::Null, KeyModifiers::NONE);
}

impl App<'_> {
    pub fn update_current_available_events(&mut self) {
        let is_there_any_env = match self.get_selected_env_as_local() {
            None => false,
            Some(_) => true
        };

        let protocol = match &self.collections_tree.selected {
            Some(selected_request_index) => {
                let local_selected_request = self.collections[selected_request_index.0].requests[selected_request_index.1].clone();
                let selected_request = local_selected_request.read();
                Some(selected_request.protocol.clone())
            },
            None => None
        };
        
        *AVAILABLE_EVENTS.write() = self.state.get_available_events(self.request_view, self.request_param_tab, protocol, is_there_any_env);
    }

    pub fn get_state_line(&self) -> Line<'_> {
        match self.state {
            Normal |
            ChoosingElementToCreate |
            CreatingNewCollection | CreatingNewRequest |
            DisplayingCookies | EditingCookies |
            DisplayingLogs => Line::from(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color),

            DeletingCollection | RenamingCollection => {
                let collection_index = self.collections_tree.state.selected()[0];
                let collection_name = &self.collections[collection_index].name;

                Line::from(vec![
                    Span::raw(t!("Collection > ")).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(format!("{} > ", collection_name)).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                ])
            },

            DeletingRequest | RenamingRequest => {
                let selected_request_index = &self.collections_tree.state.selected();
                let selected_request = &self.collections[selected_request_index[0]].requests[selected_request_index[1]].read();

                Line::from(vec![
                    Span::raw(t!("Request > ")).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(format!("{} > ", selected_request.name)).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                ])
            },

            DisplayingEnvEditor | EditingEnvVariable => {
                let local_env = self.get_selected_env_as_local().unwrap();
                let env = local_env.read();

                Line::from(vec![
                    Span::raw(t!("Environment editor > ")).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(env.name.clone()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                ])
            },

            SelectedRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthBasicUsername | EditingRequestAuthBasicPassword |
            EditingRequestAuthBearerToken |
            EditingRequestAuthJwtSecret | EditingRequestAuthJwtPayload |
            EditingRequestAuthDigestUsername | EditingRequestAuthDigestPassword | EditingRequestAuthDigestDomains | EditingRequestAuthDigestRealm | EditingRequestAuthDigestNonce | EditingRequestAuthDigestOpaque |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyFile | EditingRequestBodyString |
            EditingRequestMessage |
            EditingPreRequestScript | EditingPostRequestScript |
            EditingRequestSettings |
            ChoosingRequestExportFormat | DisplayingRequestExport
            => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read();

                if self.state == SelectedRequest {
                    Line::from(vec![
                        Span::raw(t!("Request > ")).fg(THEME.read().ui.secondary_foreground_color),
                        Span::raw(selected_request.name.clone()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                    ])
                }
                else {
                    Line::from(vec![
                        Span::raw(t!("Request > ")).fg(THEME.read().ui.secondary_foreground_color),
                        Span::raw(format!("{} > ", selected_request.name)).fg(THEME.read().ui.secondary_foreground_color),
                        Span::raw(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                    ])
                }
            }
        }
    }
    
    pub fn in_input(&self) -> bool {
        match self.state {
            EditingEnvVariable |
            EditingCookies |
            CreatingNewCollection |
            CreatingNewRequest |
            RenamingCollection |
            RenamingRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthBasicUsername | EditingRequestAuthBasicPassword | EditingRequestAuthBearerToken | EditingRequestAuthJwtSecret | EditingRequestAuthJwtPayload |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyFile | EditingRequestBodyString |
            EditingPreRequestScript | EditingPostRequestScript |
            EditingRequestSettings => true,
            _ => false
        }
    }
}
