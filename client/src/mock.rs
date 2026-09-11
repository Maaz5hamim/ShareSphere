use crate::request::{RequestType, Response, files, my_files, my_folders};

pub const DEMO_EMAIL: &str = "demo@sharesphere.com";
const DEMO_PASSWORD: &str = "demo123";

pub fn handle(request: RequestType, email: &str) -> Response {
    match request {
        RequestType::Login { password } => {
            if email == DEMO_EMAIL && password == DEMO_PASSWORD {
                Response::Success
            } else {
                Response::Failure(format!(
                    "Demo credentials: {}  /  {}",
                    DEMO_EMAIL, DEMO_PASSWORD
                ))
            }
        }
        RequestType::Signup { .. } => {
            Response::Failure("Signup is disabled in demo mode".to_string())
        }
        RequestType::UploadFile { .. } => Response::Success,
        RequestType::DownloadFile { .. } => Response::Success,
        RequestType::DeleteFile { .. } => Response::Success,
        RequestType::Share { .. } => Response::Success,
        RequestType::MyUploadList => Response::MyUploadListSuccess {
            list: demo_uploads(),
        },
        RequestType::GetFolderNames => Response::GetFolderNameSuccess {
            foldernames: demo_folders(),
        },
        RequestType::GetFileNames { folder_name } => Response::GetFileNameSuccess {
            filenames: demo_folder_files(&folder_name),
        },
    }
}

fn demo_uploads() -> Vec<my_files> {
    vec![
        my_files {
            name: "Resume_2024.pdf".to_string(),
            size: 245760,
            upload_date: "2024-11-15".to_string(),
            members: None,
        },
        my_files {
            name: "Project_Demo.mp4".to_string(),
            size: 52428800,
            upload_date: "2024-12-01".to_string(),
            members: Some("alice@example.com, bob@example.com".to_string()),
        },
        my_files {
            name: "Architecture_Diagram.png".to_string(),
            size: 1048576,
            upload_date: "2024-12-08".to_string(),
            members: Some("alice@example.com".to_string()),
        },
    ]
}

fn demo_folders() -> Vec<my_folders> {
    vec![
        my_folders {
            name: "Alice".to_string(),
            items: 2,
            updated: "2024-12-10".to_string(),
        },
        my_folders {
            name: "Bob".to_string(),
            items: 1,
            updated: "2024-11-28".to_string(),
        },
    ]
}

fn demo_folder_files(folder_name: &str) -> Vec<files> {
    match folder_name {
        "Alice" => vec![
            files {
                name: "Meeting_Notes.docx".to_string(),
                size: 32768,
                upload_date: "2024-12-10".to_string(),
            },
            files {
                name: "Q4_Budget.xlsx".to_string(),
                size: 65536,
                upload_date: "2024-12-05".to_string(),
            },
        ],
        "Bob" => vec![files {
            name: "Shared_Codebase.zip".to_string(),
            size: 10485760,
            upload_date: "2024-11-28".to_string(),
        }],
        _ => vec![],
    }
}
