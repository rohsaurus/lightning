import os
import time
import typesense
import magic
import concurrent.futures
import logging

# Set up logging
logging.basicConfig(level=logging.INFO)

# ... (Typesense client and schema setup)
# Creating the Typesense client
client = typesense.Client({
  'nodes': [{
    'host': 'localhost',  # Update with your Typesense server details
    'port': '8108',       # Update with the appropriate port
    'protocol': 'http'    # Use 'https' for secure connections
  }],
  'api_key': 'GkxVS70yMTFS7RJ9BEz6sjpPG6Ij7kdqTx7tGLIZ0AM1fJ4X',  # Replace with your Typesense API key
  'connection_timeout_seconds': 2
})

file_schema = {
  'name': 'files',
  'fields': [
    {'name': 'file_name', 'type': 'string' },
    {'name': 'date_created', 'type': 'string'},
    {'name': 'date_modified', 'type': 'string'},
    {'name': 'file_type', 'type': 'string', 'facet': True},
    {'name': 'file_location', 'type': 'string'}
  ],
}
client.collections.create(file_schema)
# ... (index_file function)
# Function to index a file
def index_file(file_path):
    file_name = os.path.basename(file_path)
    file_stat = os.stat(file_path)
    file_created = time.ctime(file_stat.st_ctime)
    file_modified = time.ctime(file_stat.st_mtime)
    file_type = magic.from_buffer(open(file_path, 'rb').read(2048),mime=True)

   # Create a document object
    document = {
        'file_name': file_name,
        'date_created': file_created,
        'date_modified': file_modified,
        'file_type': file_type,
        'file_location': file_path
    }

    # Index the document
    client.collections['files'].documents.create(document=document)
    print(f"Indexed file: {file_path}")


def index_files_in_directory(directory):
    excluded_dirs = ['build', 'temp', 'cache','bin','target','debug','release','dist','build','node_modules','venv','__pycache__', '.git', '.idea', '.vscode', '.pytest_cache', '.mypy_cache', '.cache', '.eggs', '.tox', '.ipynb_checkpoints', '.pytest_cache', '.mypy_cache', '.cache', '.eggs', '.tox', '.ipynb_checkpoints','lib','.config','.local','.vs','packages','examples','OneDrive Personal', 'OneDrive MCVTS','.var','.steam','miniconda3', 'fluttersdk','.zoom']
    excluded_extensions = ('.o', '.a', '.so','.h','.pyc','.cfg','.class','.dll','.gitattributes','.gitignore','.gitmodules','.gitkeep','.gitlab-ci.yml','.gitpod.yml','.gitpod.Dockerfile','.gitpod.dockerfile','.gitpod','.filters','.in','.ico','.settings','.csproj','.resx','.sln','.cat','.c','.cpp','.cxx','.h','.hxx','.hpp','.h++','.sys','.cat','.inf','.service','.build','.rst','CMakeLists.txt','.cmake','.natvis','.ttf','.natstepfilter','.lua','.py','.jar','.java','.js','.ts','.enc','.bin','.toml','.lock')
    files_to_index = []

    for root, dirs, files in os.walk(directory):
        dirs[:] = [d for d in dirs if d not in excluded_dirs]

        for file in files:
            if file.endswith(excluded_extensions):
                continue
            files_to_index.append(os.path.join(root, file))
    
    return files_to_index

def index_batch(batch):
    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = {executor.submit(index_file, file_path): file_path for file_path in batch}
        for future in concurrent.futures.as_completed(futures):
            file_path = futures[future]
            try:
                future.result()
                logging.info(f"Indexed file: {file_path}")
            except Exception as e:
                logging.error(f"Failed to index {file_path}: {e}")

def main(root_dir):
    batch_size = 10  # Adjust this based on your system's capabilities
    files_to_index = index_files_in_directory(root_dir)

    for i in range(0, len(files_to_index), batch_size):
        batch = files_to_index[i:i + batch_size]
        index_batch(batch)

if __name__ == "__main__":
    root_dir = "/home/rohan/"
    main(root_dir)
