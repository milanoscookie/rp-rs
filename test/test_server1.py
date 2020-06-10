from flask import Flask

app = Flask(__name__)

@app.route('/')
def index():
    return 'Hello world from 8081'
@app.route('/test')
def test():
    return 'Hello test from 8081'

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port='8081')
