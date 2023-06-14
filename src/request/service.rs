use std::path::Path;

use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::fs::{self,File};

use super::{RequestType,Response,Request};


pub async fn login(email: &str,password: &str) -> io::Result<Response> 
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for user verification
    let request_type = RequestType::Login { password: String::from(password)};
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    writer.shutdown().await?;

    //Check if the server has sent any confirmation
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();
    Ok(response)
}

pub async fn signup(email: &str, name : &str, password: &str) -> io::Result<Response> 
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();


    //sending a request to server for user registration
    let request_type = RequestType::Signup { name: String::from(name), password: String::from(password)};
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    writer.shutdown().await?;

    //Check if the server has sent any confirmation
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();
    Ok(response)
}

pub async fn upload_file(filepath: &str, email:&str) -> io::Result<Response> 
{
    // Open the file to be sent
    let mut file = File::open(filepath).await?;
    let filename = Path::new(filepath).file_name().unwrap().to_str().unwrap();

    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for file upload
    let request_type = RequestType::UploadFile {file_path : String::from(filename)};
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    // Check if server has received the request
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    // Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();

    if let Response::Initiate = response
    {
        // Read the file in chunks and send it over the network
        let mut buffer = [0; 10000];
        loop 
        {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0
            {
                // End of file reached
                break;
            }
            writer.write_all(&buffer[..bytes_read]).await?;
        }
        writer.shutdown().await?;
    }
    else{return Ok(Response::Failure("Server timeout".to_string()));}

    //Check if the server has sent any data
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();
    Ok(response)

}

pub async fn download_file(filename: &str, email: &str, filepath: &str) -> io::Result<Response> 
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for file upload
    let request_type = RequestType::DownloadFile {file_name: String::from(filename), file_path: String::from(filepath)};
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    // Check if server has received the request
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    // Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();

    if let Response::Initiate = response
    {
        //create a new file in user's directory
        let filepath = format!("{filepath}//{filename}");
        let mut file = File::create(&filepath).await?;
        // Read the file contents from the server
        let mut buffer = [0; 10000];
        loop 
        {
            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read == 0 
            {
                // all contents read
                break;
            }
            // Save the file contents to file created
            file.write_all(&mut buffer[..bytes_read]).await?;
        }

        println!("File received and saved to D://ShareSphere//{filename}");
        Ok((Response::Success))
    }
    else{return Ok(Response::Failure("Server timeout".to_string()));}

}

pub async fn get_file_name(email: &str, folder_name:&str) -> io::Result<Response>  
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for list of files shared with user
    let request_type = RequestType::GetFileNames {folder_name : String::from(folder_name)};
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    writer.shutdown().await?;

    //Wait and receive server's response
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();

    Ok(response)
}

pub async fn get_folder_name(email: &str) -> io::Result<Response>  
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for list of people who have shared files with user
    let request_type = RequestType::GetFolderNames;
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    writer.shutdown().await?;

    //Wait and receive server's response
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();

    Ok(response)
}

pub async fn delete_file(email:&str, file_name:&str) -> io::Result<Response> 
{
        // Connect to the server over a TCP connection
        let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
        stream.set_nodelay(true);
        let (mut reader, mut writer) = stream.split();
    
        //sending a request to server to delete a given file
        let request_type = RequestType::DeleteFile { file_name: String::from(file_name) };
        let request = Request
        {
            service:request_type,
            email:String::from(email),
        };
        let request_bytes = bincode::serialize(&request).unwrap();
        writer.write_all(&request_bytes).await?;
    
        writer.shutdown().await?;
    
        //Check if the server has sent any confirmation
        let mut response_bytes = [0; 1000];
        let n = reader.read(&mut response_bytes).await?;
        //Deserialize the response from the server
        let response: Response = bincode::deserialize(&response_bytes).unwrap();
        println!("{:?}",response);
        Ok(response)
}

pub async fn my_upload_list(email: &str) -> io::Result<Response>  
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for list of files uploaded by user
    let request_type = RequestType::MyUploadList;
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    writer.shutdown().await?;

    //Wait and receive server's response
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();

    Ok(response)
}

pub async fn share(email: &str, members:Vec<String>, filename: &str) -> io::Result<Response>  
{
    // Connect to the server over a TCP connection
    let mut stream = TcpStream::connect("192.168.43.65:8080").await?;
    stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.split();

    //sending a request to server for list of members to share files with
    let request_type = RequestType::Share { members, filename: String::from(filename) };
    let request = Request
    {
        service:request_type,
        email:String::from(email),
    };
    let request_bytes = bincode::serialize(&request).unwrap();
    writer.write_all(&request_bytes).await?;

    writer.shutdown().await?;

    //Wait and receive server's response
    let mut response_bytes = [0; 1024];
    let n = reader.read(&mut response_bytes).await?;
    //Deserialize the response from the server
    let response: Response = bincode::deserialize(&response_bytes).unwrap();

    Ok(response)
}