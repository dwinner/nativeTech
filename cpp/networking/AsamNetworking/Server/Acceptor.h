#pragma once

#include <boost/asio.hpp>
#include <atomic>
#include <memory>

namespace asamNetworking::server
{
    /**
     * Responsible for accepting the connection requests arriving from clients and instantiating the objects of the Service class,
     * which will provide the service to connected clients.
     */
    class Acceptor
    {
    public:
        /**
         * Accepts a port number on which it will listen for the incoming connection requests as its input argument.
         * @param anIoService I/O service
         * @param aPortNumber Port number
         */
        Acceptor(boost::asio::io_service& anIoService, unsigned short aPortNumber);

        /**
         * Intended to instruct an object of the Acceptor class to start listening and accepting incoming connection requests.
         */
        void Start(const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn);

        /**
         * Stop accepting incoming connection requests.
         */
        void Stop();

    private:
        void InitAccept(std::function<std::string(const boost::asio::streambuf&)> proceccReqFn);

        void OnAccept(const boost::system::error_code& anErrorCode,
                      const std::shared_ptr<boost::asio::ip::tcp::socket>& aSocket,
                      const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn);

        boost::asio::io_service& ioService_;
        // used to asynchronously accept the incoming connection requests.
        boost::asio::ip::tcp::acceptor acceptor_;
        std::atomic<bool> isStopped_;
    };
}
