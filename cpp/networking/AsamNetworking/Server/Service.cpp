#include "pch.h"
#include "Service.h"
#include <iostream>
#include "Buffer.h"
#include "../Helpers.h"

namespace asamNetworking::server
{
    using namespace boost;

    Service::Service(std::shared_ptr<asio::ip::tcp::socket> aSocket)
        : socket_(std::move(aSocket))
    {
    }

    void Service::StartHandling(std::function<std::string(const asio::streambuf&)> proceccReqFn)
    {
        asio::async_read_until(*socket_,
                               request_,
                               EndOfProtocolMarker,
                               [this, proceccReqFn](const system::error_code& anErrorCode, const std::size_t bytesTransferred)
                               {
                                   // When the request reading completes, or an error occurs,
                                   // the callback method onRequestReceived() is called.
                                   OnRequestReceived(anErrorCode, bytesTransferred, proceccReqFn);
                               });
    }

    void Service::OnRequestReceived(
        const system::error_code& anErrorCode,
        std::size_t bytesTransferred,
        const std::function<std::string(const asio::streambuf&)>& proceccReqFn)
    {
        // This method first checks whether the reading succeeded by testing the ec argument that contains the operation completion status
        // code.
        if (anErrorCode.value() != 0)
        {
            std::cout << "Error occured! Error code = " << anErrorCode.value() << ". Message: " << anErrorCode.message();

            // reading finished with an error, the corresponding message is output to the standard output stream
            // and then the onFinish() method is called.
            OnFinish();
            return;
        }

        // Process the request.
        response_ = ProcessRequest(request_, proceccReqFn);

        // When the ProcessRequest() method completes and returns the string containing the response message,
        // the asynchronous writing operation is initiated to send this response message back to the client.
        asio::async_write(*socket_,
                          asio::buffer(response_),
                          [this](const system::error_code& errorCode, const std::size_t transferred)
                          {
                              // The onResponseSent() method is specified as a callback.
                              OnResponseSent(errorCode, transferred);
                          });
    }

    void Service::OnResponseSent(const system::error_code& anErrorCode, std::size_t bytesTransferred)
    {
        // This method first checks whether the operation succeeded.
        if (anErrorCode.value() != 0)
        {
            // If the operation failed, the corresponding message is output to the standard output stream.
            std::cout << "Error occured! Error code = " << anErrorCode.value() << ". Message: " << anErrorCode.message();
        }

        // method is called to perform the cleanup.
        OnFinish();
    }

    std::string Service::ProcessRequest(
        const asio::streambuf& aRequest,
        const std::function<std::string(const asio::streambuf&)>& proceccReqFn)
    {
        // TONOTE: Server's request/response processing.

        const std::string response = proceccReqFn(aRequest);
        return response + std::string{EndOfProtocolMarker};
    }
}
