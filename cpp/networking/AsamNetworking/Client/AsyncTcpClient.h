#pragma once

#include <boost/predef.h>

#ifdef BOOST_OS_WINDOWS
#define _WIN32_WINNT 0x0501

#if _WIN32_WINNT <= 0x0502 // Windows Server 2003 or earlier.
#define BOOST_ASIO_DISABLE_IOCP
#define BOOST_ASIO_ENABLE_CANCELIO
#endif
#endif

#include <boost/asio.hpp>
#include <boost/noncopyable.hpp>

#include <thread>
#include <mutex>
#include <memory>
#include <list>
#include <map>

#include "Session.h"

namespace asamNetworking::client
{
    /**
     * Class that provides the asynchronous communication functionality.
     */
    class AsyncTcpClient : public boost::noncopyable
    {
    public:
        AsyncTcpClient(unsigned char numOfThreads);

        /**
         * Initiates a request to the server
         *
         * @param rawIpAddress IP address
         * @param portNum Port number
         * @param callback Callback function for handling response
         * @param requestId Request ID
         * @param aRequest Request body
         */
        void ApplyRequest(
            const std::string& rawIpAddress,
            unsigned short portNum,
            Callback callback,
            unsigned int requestId,
            const std::string& aRequest);

        /**
         * Cancels the previously initiated request designated by the request_id argument
         * @param requestId Identifier of the request to be canceled
         */
        void CancelRequest(unsigned int requestId);

        /**
         * Blocks the calling thread until all the currently running requests complete and deinitializes the client.
         */
        void Close();

    private:
        // method is called whenever the request completes with any result.
        void OnRequestComplete(const std::shared_ptr<Session>& session);

        boost::asio::io_service ioService_;
        std::map<int, std::shared_ptr<Session>> activeSessions_;
        std::mutex activeSessionsGuard_;
        std::unique_ptr<boost::asio::io_service::work> work_;
        std::list<std::unique_ptr<std::thread>> threads_;
    };
}
