#pragma once

#include <boost/asio.hpp>
#include <memory>

namespace asamNetworking::server
{
    /**
     * Responsible for handling a single client by reading the request message, processing it,
     * and then sending back the response message.
     * Each instance of the Service class is intended to handle one connected client
     * by reading the request message, processing it, and then sending the response message back.
     */
    class Service
    {
    public:
        /**
         * Accepts a shared pointer to an object representing a socket connected to a particular client as an argument
         * and caches this pointer. This socket will be used later to communicate with the client application.
         * @param aSocket Socket's shared ponter
         */
        explicit Service(std::shared_ptr<boost::asio::ip::tcp::socket> aSocket);

        /**
         * Starts handling the client by initiating the asynchronous reading operation
         * to read the request message from the client specifying the onRequestReceived() method as a callback.
         */
        void StartHandling(std::function<std::string(const boost::asio::streambuf&)> proceccReqFn);

    private:
        void OnRequestReceived(const boost::system::error_code& anErrorCode,
                               std::size_t bytesTransferred,
                               const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn);

        void OnResponseSent(const boost::system::error_code& anErrorCode, std::size_t bytesTransferred);

        // Here we perform the cleanup.
        void OnFinish() const
        {
            delete this;
        }

        /**
         * Process request
         * @param aRequest Request
         * @param proceccReqFn Function to process request
         * @return Response
         */
        std::string ProcessRequest(const boost::asio::streambuf& aRequest,
                                   const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn);

        std::shared_ptr<boost::asio::ip::tcp::socket> socket_;
        std::string response_;
        boost::asio::streambuf request_;
    };
}
