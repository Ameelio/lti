//! Canvas specific extensions to LTI.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanvasDisplayType {
    Borderless,
    Default,
    FullWidth,
    FullWidthInContext,
    FullWidthWithNav,
    InNavContext,
    NewWindow,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CanvasLaunchMeasurement {
    Number(u16),
    String(Box<str>),
}

/// Canvas specific placements - locations where the tool may render output to.
/// See [Placements Overview](https://developerdocs.instructure.com/services/canvas/external-tools/lti/placements/file.placements_overview)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CanvasPlacement {
    #[serde(rename = "https://canvas.instructure.com/lti/account_navigation")]
    AccountNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/assignment_edit")]
    AssignmentEdit,
    #[serde(rename = "https://canvas.instructure.com/lti/assignment_group_menu")]
    AssignmentGroupMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/assignment_index_menu")]
    AssignmentIndexMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/assignment__menu")]
    AssignmentMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/assignment_selection")]
    AssignmentSelection,
    //    #[serde(rename = "https://canvas.instructure.com/lti/assignment_view")]
    #[serde(rename = "assignment_view")]
    AssignmentView,
    #[serde(rename = "https://canvas.instructure.com/lti/collaboration")]
    Collaboration,
    #[serde(rename = "https://canvas.instructure.com/lti/course_assignments_menu")]
    CourseAssignmentsMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/course_home_sub_navigation")]
    CourseHomeSubNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/course_navigation")]
    CourseNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/course_settings_sub_navigation")]
    CourseSettingsSubNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/discussion_topic_index_menu")]
    DiscussionTopicIndexMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/discussion_topic_menu")]
    DiscussionTopicMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/editor_button")]
    EditorButton,
    #[serde(rename = "https://canvas.instructure.com/lti/file_index_menu")]
    FileIndexMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/file_menu")]
    FileMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/global_navigation")]
    GlobalNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/homework_submission")]
    HomeworkSubmission,
    #[serde(rename = "https://canvas.instructure.com/lti/link_selection")]
    LinkSelection,
    #[serde(rename = "https://canvas.instructure.com/lti/migration_selection")]
    MigrationSelection,
    #[serde(rename = "https://canvas.instructure.com/lti/module_group_menu")]
    ModuleGroupMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/module_index_menu_modal")]
    ModuleIndexMenuModal,
    #[serde(rename = "https://canvas.instructure.com/lti/module_index_menu_tray")]
    ModuleIndexMenuTray,
    #[serde(rename = "https://canvas.instructure.com/lti/module_index_modal")]
    ModuleIndexModal,
    #[serde(rename = "https://canvas.instructure.com/lti/module_menu")]
    ModuleMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/post_grades")]
    PostGrades,
    #[serde(rename = "https://canvas.instructure.com/lti/quiz_index_menu")]
    QuizIndexMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/quiz_menu")]
    QuizMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/student_context_card")]
    StudentContextCard,
    #[serde(rename = "https://canvas.instructure.com/lti/submission_type_selection")]
    SubmissionTypeSelection,
    #[serde(rename = "https://canvas.instructure.com/lti/tool_configuration")]
    ToolConfiguration,
    #[serde(rename = "https://canvas.instructure.com/lti/top_navigation")]
    TopNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/user_navigation")]
    UserNavigation,
    #[serde(rename = "https://canvas.instructure.com/lti/wiki_index_menu")]
    WikiIndexMenu,
    #[serde(rename = "https://canvas.instructure.com/lti/wiki_page_menu")]
    WikiPageMenu,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanvasVisibility {
    Admins,
    Members,
    Public,
}
