from flask import Flask, request
from threading import Thread
import stomp
import time
from datetime import datetime
import threading

# default localhost or configure it on ip address change ip address if network changes 
ActiveMQ_IP = "192.169.7.279"

app = Flask(__name__)
@app.route('/')
def hello():
    return 'Hello, World!'

@app.route('/send_message')
def send_message():
    my_thread = threading.Thread()
    my_thread.start()

    queue = request.args.get('queue', 'testMauq')
    message = request.args.get('message', '.')
    author = request.args.get('author', '-')
    now = datetime.now()
    formatted_time = now.strftime("%Y-%m-%d %H:%M:%S")
    date_time = request.args.get('date_time', formatted_time)
    # name = request.args.get('value', 'default')
    
    # conn = stomp.Connection([('172.20.195.124', 61613)])
    conn = stomp.Connection([(ActiveMQ_IP, 61613)])
    conn.connect()

    message = {
        "queue" : queue,
        "message" : message,
        "author" : author,
        "date_time" : date_time
    }
    print(message)
    conn.send(queue,str(message))

    conn.disconnect()
    # my_thread.join()

    return 'Message sent'

class MyListener(object):
    msg_list = []

    def __init__(self):
        self.msg_list = []

    def on_error(self, headers, message):
        print(headers,message)
        self.msg_list.append('(ERROR) ' , message)

    def on_message(self, headers, message):
        print(message)
        self.msg_list.append(message)


@app.route('/consume_message')
def consume_message():
    my_thread = threading.Thread()
    my_thread.start()

    queue = request.args.get('queue', 'testMauq')
    # conn = stomp.Connection([('172.20.195.124', 61613)])
    conn = stomp.Connection([(ActiveMQ_IP, 61613)])
    lst = MyListener()
    conn.set_listener('', lst)
    conn.start()
    conn.connect()
    conn.subscribe(destination=queue, id=1, ack='auto')
    time.sleep(2)
    messages = lst.msg_list
    conn.disconnect()

    # my_thread.join()
    print(messages)

    return ",".join(messages)



if __name__ == '__main__':
    # Start a thread for the Flask app
    # t = Thread(target=app.run)
    t = Thread()
    t.start()
    app.run(threaded=True, debug = True)
