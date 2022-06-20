# copy_to_output
A rust library to copy files/folders from the project directory to the output directory

## Copy folder
`copy_folder("folder_name");`

## Copy file
`copy_file("file_name");`

To do this during the build, create a build.rs file and add either copy_folder() or copy_file().
