#define WIN32_LEAN_AND_MEAN

#include <boost/asio.hpp>

#include <thread>
#include <iostream>
#include <string>
#include "can_payload.h"
#include "Server/Buffer.h"
#include "Server/Server.h"

constexpr unsigned int DefaultThrPoolSize = 2;

namespace
{
    std::function ProceccReqFn = [](const boost::asio::streambuf& request)
    {
        using namespace ASAM::CMP;

        auto payload = asamNetworking::server::BufferToVector(request);
        const auto canPayload = std::make_unique<CanPayload>(payload.data(), payload.size() - 1);
        const CanPayloadBase* payLoadBase = canPayload.get();
        const PayloadType payLoadType = payLoadBase->getType();
        const std::string response = payLoadType == PayloadType::can ? "This is a CAN payload" : "This is not a CAN payload";

        return response;
    };
}

int main()
{
    try
    {
        using namespace asamNetworking::server;

        constexpr unsigned short portNum = 3333;

        // it instantiates an object of the Server class named srv.
        Server server;

        // before starting the server, the optimal size of the pool is calculated.
        // The general formula often used in parallel applications to find the optimal number of threads is the number of processors the
        // computer has multiplied by 2. We use the std::thread::hardware_concurrency() static method to obtain the number of processors.
        unsigned int threadPoolSize = std::thread::hardware_concurrency() * 2;

        // because this method may fail to do its job returning 0,
        // we fall back to default value represented by the constant DEFAULT_THREAD_POOL_SIZE, which is equal to 2 in our case.
        if (threadPoolSize == 0)
        {
            threadPoolSize = DefaultThrPoolSize;
        }

        server.Start(portNum, threadPoolSize, ProceccReqFn);
        std::this_thread::sleep_for(std::chrono::seconds(3'600));
        server.Stop();
    }
    catch (boost::system::system_error& systemError)
    {
        std::cout << "Error occured! Error code = " << systemError.code() << ". Message: " << systemError.what();
    }

    return 0;
}
