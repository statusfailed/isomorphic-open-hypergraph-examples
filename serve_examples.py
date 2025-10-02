#!/usr/bin/env python3

import os
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import mimetypes

class ExampleHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        parsed_path = urlparse(self.path)
        path = parsed_path.path
        
        if path == '/':
            self.serve_index()
        elif path.startswith('/example_isomorphisms/'):
            self.serve_file(path[1:])
        elif path == '/api/examples':
            self.serve_examples_json()
        else:
            self.send_error(404)
    
    def serve_index(self):
        html = self.generate_index_html()
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(html.encode())
    
    def serve_file(self, file_path):
        try:
            with open(file_path, 'rb') as f:
                content = f.read()
            
            mime_type, _ = mimetypes.guess_type(file_path)
            if mime_type is None:
                mime_type = 'application/octet-stream'
            
            self.send_response(200)
            self.send_header('Content-type', mime_type)
            self.end_headers()
            self.wfile.write(content)
        except FileNotFoundError:
            self.send_error(404)
    
    def serve_examples_json(self):
        examples = self.get_examples()
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps(examples).encode())
    
    def get_examples(self):
        examples = []
        examples_dir = 'example_isomorphisms'
        
        if os.path.exists(examples_dir):
            for example_name in os.listdir(examples_dir):
                example_path = os.path.join(examples_dir, example_name)
                if os.path.isdir(example_path):
                    example = {
                        'name': example_name,
                        'files': {}
                    }
                    
                    for file_name in os.listdir(example_path):
                        file_path = os.path.join(example_path, file_name)
                        if os.path.isfile(file_path):
                            if file_name.endswith('.svg'):
                                example['files'][file_name] = f'/{example_path}/{file_name}'
                            elif file_name.endswith('.json'):
                                example['files'][file_name] = f'/{example_path}/{file_name}'
                    
                    examples.append(example)
        
        return examples
    
    def generate_index_html(self):
        examples = self.get_examples()
        
        html = '''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Isomorphic Open Hypergraph Examples</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 20px;
            background-color: #f5f5f5;
        }
        h1 {
            color: #333;
            text-align: center;
        }
        table {
            width: 100%;
            border-collapse: collapse;
            background-color: white;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        th, td {
            padding: 15px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        th {
            background-color: #4a4a4a;
            color: white;
        }
        .svg-container {
            max-width: 400px;
            margin: 10px 0;
        }
        .svg-container svg {
            max-width: 100%;
            height: auto;
        }
        .json-toggle {
            background-color: #007bff;
            color: white;
            border: none;
            padding: 5px 10px;
            cursor: pointer;
            border-radius: 3px;
            margin: 5px 0;
        }
        .json-toggle:hover {
            background-color: #0056b3;
        }
        .json-content {
            display: none;
            background-color: #f8f9fa;
            border: 1px solid #dee2e6;
            border-radius: 3px;
            padding: 10px;
            margin: 10px 0;
            white-space: pre-wrap;
            font-family: monospace;
            font-size: 12px;
            max-height: 200px;
            overflow-y: auto;
        }
        .example-cell {
            vertical-align: top;
        }
    </style>
</head>
<body>
    <h1>Isomorphic Open Hypergraph Examples</h1>
    <table>
        <thead>
            <tr>
                <th>Example Name</th>
                <th>Hypergraph A</th>
                <th>Hypergraph B</th>
            </tr>
        </thead>
        <tbody>'''
        
        for example in examples:
            html += f'''
            <tr>
                <td class="example-cell"><strong>{example['name']}</strong></td>'''
            
            for suffix in ['a', 'b']:
                svg_file = f'{suffix}.svg'
                json_file = f'{suffix}.json'
                
                html += f'''
                <td class="example-cell">'''
                
                if svg_file in example['files']:
                    html += f'''
                    <div class="svg-container">
                        <object data="{example['files'][svg_file]}" type="image/svg+xml">
                            SVG not supported
                        </object>
                    </div>'''
                
                if json_file in example['files']:
                    json_id = f"{example['name']}--{suffix}--json"
                    html += f'''
                    <button class="json-toggle" onclick="toggleJson('{json_id}')">
                        See JSON
                    </button>
                    <div id="{json_id}" class="json-content"></div>'''
                
                html += '</td>'
            
            html += '</tr>'
        
        html += '''
        </tbody>
    </table>
    
    <script>
        async function toggleJson(jsonId) {
            const jsonDiv = document.getElementById(jsonId);
            const button = document.querySelector(`button[onclick="toggleJson('${jsonId}')"]`);
            
            if (jsonDiv.style.display === 'none' || jsonDiv.style.display === '') {
                if (jsonDiv.innerHTML === '') {
                    // Load JSON content
                    const [exampleName, suffix] = jsonId.replace('--json', '').split('--');
                    try {
                        const response = await fetch(`/example_isomorphisms/${exampleName}/${suffix}.json`);
                        const jsonData = await response.json();
                        jsonDiv.innerHTML = JSON.stringify(jsonData, null, 2);
                    } catch (error) {
                        jsonDiv.innerHTML = 'Error loading JSON: ' + error.message;
                    }
                }
                jsonDiv.style.display = 'block';
                button.textContent = 'Hide JSON';
            } else {
                jsonDiv.style.display = 'none';
                button.textContent = 'See JSON';
            }
        }
    </script>
</body>
</html>'''
        
        return html

def main():
    port = 8000
    server_address = ('', port)
    httpd = HTTPServer(server_address, ExampleHandler)
    
    print(f"Serving at http://localhost:{port}")
    print("Press Ctrl+C to stop the server")
    
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopped.")
        httpd.server_close()

if __name__ == '__main__':
    main()