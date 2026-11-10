#pragma once

#include <boost/asio.hpp>

namespace asamNetworking::client
{
    // Function pointer type that points to the callback
    // function which is called when a request is complete.
    // Based on the values of the parameters passed to it, it outputs information about the finished request.
    typedef void (*Callback)(unsigned int aRequestId,
                             // unique identifier of the request is assigned to the request when it was initiated.
                             const std::string& aResponse,
                             // the response data
                             const boost::system::error_code& anErrorCode); // error information
}
