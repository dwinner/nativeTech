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
#include <mutex>
#include "Common.h"

namespace asamNetworking::client
{
    /**
     * Data structure whose purpose is to keep the data related to a particular request while it is being executed
     */
    struct Session
    {
        Session(boost::asio::io_service& anIoService,
                const std::string& rawIpAddress,
                const unsigned short portNum,
                std::string request,
                const unsigned int id,
                const Callback callback)
            : Socket(anIoService)
              , Endpoint(boost::asio::ip::address::from_string(rawIpAddress), portNum)
              , Request(std::move(request))
              , Id(id)
              , Callback(callback)
              , WasCancelled(false)
        {
        }

        boost::asio::ip::tcp::socket Socket; // Socket used for communication
        boost::asio::ip::tcp::endpoint Endpoint; // Remote endpoint.
        std::string Request; // Request string.

        // streambuf where the response will be stored.
        boost::asio::streambuf ResponseBuf;
        std::string Response; // Response represented as a string.

        // Contains the description of an error if one occurs during
        // the request lifecycle.
        boost::system::error_code ErrorCode;

        unsigned int Id; // Unique ID assigned to the request.

        // Pointer to the function to be called when the request
        // completes.
        Callback Callback;

        bool WasCancelled;
        std::mutex CancelGuard;
    };
}
