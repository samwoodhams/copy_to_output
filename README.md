# copy_to_output
A rust library to copy files/folders from the project directory to the output directory

## Copy file
`copy_file("file_name");`

## Copy folder
`copy_folder("folder_name");`

**To copy during build, create a build.rs file in the project root directory and add either copy_folder() or copy_file().**
