use std::f32::consts::E;

use serde::{Serialize, Deserialize};

pub mod service;

use service::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct files
{
    pub name: String,
    pub size: i32,
    pub upload_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct my_folders
{
    pub name: String,
    pub items: i32,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct my_files
{
    pub name: String,
    pub size: i32,
    pub upload_date: String,
    pub members : Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request
{
    service:RequestType,
    email:String

}
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestType 
{
    Login {password: String },
    Signup{name:String, password:String},
    UploadFile { file_path: String},
    DownloadFile { file_name: String, file_path: String},
    DeleteFile {file_name:String},
    MyUploadList,
    GetFileNames {folder_name: String},
    GetFolderNames,
    Share {members: Vec<String>, filename:String},
}
#[derive(Debug, Serialize,  Deserialize)]
pub enum Response 
{
    Success,
    Failure(String), 
    GetFileNameSuccess { filenames: Vec<files> },
    GetFolderNameSuccess { foldernames: Vec<my_folders>},
    MyUploadListSuccess { list: Vec<my_files>},
    Initiate,
}

pub async fn new(request:RequestType, email:&str) -> Response
{
    match request
    {
        RequestType::Login { password } => 
        {
            let result = login(&email, &password).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("Invalid email/password".to_string())}
                    else{response}
                },
                
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::Signup { name, password } => 
        {
            let result = signup(&email, &name, &password).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("Account exists".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::UploadFile { file_path } => 
        {
            let result = upload_file(&file_path, &email).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("Upload Failed".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::DownloadFile { file_name, file_path }=> 
        {
            let result = download_file(&file_name, &email, &file_path).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("An error occured while downloading".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::DeleteFile { file_name } => 
        {
            let result = delete_file(email, &file_name).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("Couldn't remove file".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::GetFileNames { folder_name } => 
        {
            let result = get_file_name(email, &folder_name).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("No files to show".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::GetFolderNames => 
        {
            let result = get_folder_name(email).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("No folders to show".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::MyUploadList => 
        {
            let result = my_upload_list(email).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("No files to show".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
        RequestType::Share { members, filename } => 
        {
            let result = share(email, members, &filename).await;

            match result 
            {
                Ok(response) => 
                {
                    if let Response::Failure(e) = response {Response::Failure("File already shared with member(s)".to_string())}
                    else{response}
                },
                Err(_) => Response::Failure("Cannot connect to server".to_string()),
            }
        }
    }
}
