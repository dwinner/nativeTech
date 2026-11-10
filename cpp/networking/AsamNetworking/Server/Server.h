#pragma once

#include <boost/asio.hpp>
#include <thread>
#include <memory>
#include "Acceptor.h"

namespace asamNetworking::server
{
    /**
     * Represents the server itself
     */
    class Server
    {
    public:
        Server();

        /**
         * Start the server (Nonblocking).
         * Accepts a protocol port number on which the server should listen for the incoming connection requests
         * @param aPortNumber Port number
         * @param threadPoolSize Thread pool size, number of threads to run the event loop
         * @param proceccReqFn Function to process the request
         */
        void Start(unsigned short aPortNumber,
                   unsigned int threadPoolSize,
                   const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn);

        /**
         * Stop the server.
         * Blocks the caller thread until the server is stopped and all the threads running the event loop exit.
         */
        void Stop();

    private:
        boost::asio::io_service ioService_;
        std::unique_ptr<boost::asio::io_service::work> work_;
        std::unique_ptr<Acceptor> acceptor_;
        std::vector<std::unique_ptr<std::thread>> threadPool_;
    };
}
