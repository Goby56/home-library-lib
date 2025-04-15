from flask import Flask, request

app = Flask(__name__)

@app.route('/')
def home():
    # Get all query parameters
    params = request.args
    
    # Display each parameter and its value
    if params:
        response = '<h1>All Search Parameters</h1><ul>'
        for key, value in params.items():
            response += f'<li><strong>{key}:</strong> {value}</li>'
        response += '</ul>'
    else:
        response = '<h1>No parameters provided</h1>'
    
    return response

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=5000, debug=True)

