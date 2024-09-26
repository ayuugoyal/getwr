# getwr

**getwr** is a command-line utility developed in Rust that serves as a lightweight clone of `wget`. It allows users to efficiently download files from the web while providing a user-friendly experience with progress indication.

## Features

- Download files from a specified URL.
- Display progress during the download process using a progress bar.
- Handle HTTP requests and responses seamlessly.
- Recognize and display content type and length of the downloaded files.
- Overwrite existing files with prompts or save them under a new name if a file with the same name already exists.

## Requirements

- Rust (1.50.0 or later)
- Cargo (Rust package manager)

## Installation

To install `getwr`, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/ayuugoyal/getwr.git
cd getwr
cargo build --release
```
The binary will be located in the target/release directory.

## Usage

To use getwr, run the following command in your terminal:

```bash
./target/release/getwr <URL>
```

Replace <URL> with the link to the file you wish to download.
Example

```bash
./target/release/getwr https://example.com/file.zip
```

## Options

- URL: The URL of the file to download (required).

## Progress Bar

The progress bar provides real-time feedback on the download status, displaying the elapsed time, bytes downloaded, and estimated time remaining.

## Handling Existing Files

When attempting to download a file that already exists, the program will prompt the user to:

  1. Overwrite the existing file.
  2. Save the downloaded file with a new name
  3. Exit the program
