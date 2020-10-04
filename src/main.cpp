#include "SerialPort.hpp"
#include <iostream>

// const char *portName = "\\\\.\\COM3";
#define MAX_DATA_LENGTH 255

//Declare a global object
// SerialPort *myPort;

class ConnectionHelper
{
private:
    SerialPort *_port;
    char _portName[255];

public:
    ConnectionHelper(const char *portname)
    {
        strcpy(_portName, portname);
        _port = new SerialPort(_portName);
    }

    ~ConnectionHelper() {}

    bool connect()
    {
        while (true)
        {
            // ui - searching
            std::cout << "Searching in progress";
            // wait connection
            while (!_port->isConnected())
            {
                Sleep(500);
                std::cout << ".";
                _port = new SerialPort(_portName);
            }

            //Checking if arduino is connected or not
            if (_port->isConnected())
            {
                std::cout << std::endl
                          << "Connection established at port " << _portName << std::endl;
                return true;
            }
        }
    }
};

int main()
{

    // std::cout << "Is connected: " << myPort->isConnected() << std::endl;

    return 0;
}