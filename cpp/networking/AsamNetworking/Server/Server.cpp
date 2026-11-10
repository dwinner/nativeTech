#include "pch.h"
#include "Server.h"
#include <atomic>
#include <iostream>

namespace asamNetworking::server
{
    using namespace boost;

    Server::Server()
    {
        work_.reset(new asio::io_service::work(ioService_));
    }

    void Server::Start(
        const unsigned short aPortNumber,
        const unsigned int threadPoolSize,
        const std::function<std::string(const boost::asio::streambuf&)>& proceccReqFn)
    {
        assert(threadPoolSize > 0);

        // Create and start Acceptor.
        acceptor_.reset(new Acceptor(ioService_, aPortNumber));
        acceptor_->Start(proceccReqFn);

        // Create specified number of threads and
        // add them to the pool.
        for (unsigned int i = 0; i < threadPoolSize; i++)
        {
            std::unique_ptr<std::thread> thread(new std::thread([this]
            {
                ioService_.run();
            }));
            threadPool_.push_back(std::move(thread));
        }
    }

    void Server::Stop()
    {
        acceptor_->Stop();
        ioService_.stop();
        for (const auto& thread : threadPool_)
        {
            thread->join();
        }
    }
}
