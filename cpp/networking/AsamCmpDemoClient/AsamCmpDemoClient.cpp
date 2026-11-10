#define WIN32_LEAN_AND_MEAN

#include <boost/predef.h>
#ifdef BOOST_OS_WINDOWS
#define _WIN32_WINNT 0x0501 // XP

#if _WIN32_WINNT <= 0x0502 // Windows Server 2003 or earlier.
#define BOOST_ASIO_DISABLE_IOCP
#define BOOST_ASIO_ENABLE_CANCELIO
#endif
#endif

#include <boost/asio.hpp>
#include <memory>
#include <iostream>

#include "payload_samples.h"
#include "Client/AsyncTcpClient.h"

using namespace boost;
using namespace asamNetworking::client;

namespace
{
    /**
     * Function that will serve as a callback, which we'll pass to the AsyncTCPClient::ApplyRequest() method
     * It outputs the result of the request execution and the response message to the standard output stream
     * if the request is completed successfully
     * @param requestId Request ID
     * @param response Response from the server
     * @param errorCode Error code if presented
     */
    void Handler(const unsigned int requestId, const std::string& response, const system::error_code& errorCode)
    {
        // TONOTE: Server's response on the client
        if (errorCode.value() == 0)
        {
            std::cout << "Request #" << requestId << " has completed. Response: " << response << '\n';
        }
        else if (errorCode == asio::error::operation_aborted)
        {
            std::cout << "Request #" << requestId << " has been cancelled by the user." << '\n';
        }
        else
        {
            std::cout << "Request #" << requestId << " failed! Error code = " << errorCode.value()
                << ". Error message = " << errorCode.message() << '\n';
        }
    }
}

int main()
{
    try
    {
        AsyncTcpClient client(4);

        // Here we emulate the user's behavior.
        // creates an instance of the AsyncTCPClient class and then calls its method to initiate
        // asynchronous request. User initiates a request with id 1.
        const auto canPayload = CreateCanPayload();
        client.ApplyRequest("127.0.0.1", 3333, Handler, 1, canPayload);

        // Decides to exit the application.
        std::this_thread::sleep_for(std::chrono::seconds(3'600));
        client.Close();
    }
    catch (system::system_error& sysErr)
    {
        std::cout << "Error occured! Error code: " << sysErr.code() << ". Message: " << sysErr.what();
        return sysErr.code().value();
    }

    return 0;
}
