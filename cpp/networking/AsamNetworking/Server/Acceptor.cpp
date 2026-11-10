#include "pch.h"
#include "Acceptor.h"
#include <iostream>
#include "Service.h"

namespace asamNetworking::server
{
    using namespace boost;

    Acceptor::Acceptor(asio::io_service& anIoService, const unsigned short aPortNumber)
        : ioService_(anIoService)
          , acceptor_(ioService_, asio::ip::tcp::endpoint(asio::ip::address_v4::any(), aPortNumber))
          , isStopped_(false)
    {
    }

    void Acceptor::Start(const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn)
    {
        // It puts acceptor socket into listening mode
        acceptor_.listen();
        InitAccept(proceccReqFn);
    }

    void Acceptor::Stop()
    {
        isStopped_.store(true);
    }

    void Acceptor::InitAccept(std::function<std::string(const asio::streambuf&)> proceccReqFn)
    {
        // constructs an active socket object and initiates the asynchronous accept operation
        std::shared_ptr<asio::ip::tcp::socket> sock(new asio::ip::tcp::socket(ioService_));

        // calling the async_accept() method on the acceptor socket object
        //  and passing the object representing an active socket to it as an argument.
        acceptor_.async_accept(*sock,
                               [this, sock, proceccReqFn](const system::error_code& errorCode)
                               {
                                   // When the connection request is accepted or an error occurs, the callback method onAccept() is called.
                                   OnAccept(errorCode, sock, proceccReqFn);
                               });
    }

    void Acceptor::OnAccept(
        const system::error_code& anErrorCode,
        const std::shared_ptr<asio::ip::tcp::socket>& aSocket,
        const std::function<std::string(const asio::streambuf&)>& proceccReqFn)
    {
        if (anErrorCode.value() == 0)
        {
            // an instance of the Service class is created and its StartHandling() method is called
            (new Service(aSocket))->StartHandling(proceccReqFn);
        }
        else
        {
            // the corresponding message is output to the standard output stream.
            std::cout << "Error occured! Error code = " << anErrorCode.value() << ". Message: " << anErrorCode.message();
        }

        // Init next async accept operation if acceptor has not been stopped yet.
        if (!isStopped_.load())
        {
            InitAccept(proceccReqFn);
        }
        else
        {
            // Stop accepting incoming connections and free allocated resources.
            acceptor_.close();
        }
    }
}
