#pragma once

#include <boost/asio.hpp>
#include <string>

namespace asamNetworking::server
{
    inline std::string BufferToString(const boost::asio::streambuf& buffer)
    {
        using boost::asio::buffers_begin;

        const auto data = buffer.data();
        std::string result{buffers_begin(data), buffers_end(data)};

        return result;
    }

    inline std::vector<uint8_t> BufferToVector(const boost::asio::streambuf& buffer)
    {
        using boost::asio::buffers_begin;

        std::vector<uint8_t> result(buffer.size());
        buffer_copy(boost::asio::buffer(result), buffer.data());

        return result;
    }
}
