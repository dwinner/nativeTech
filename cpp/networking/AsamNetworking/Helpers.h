#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace asamNetworking
{
    inline static char EndOfProtocolMarker = '\n';

    inline std::string PayloadToString(const std::vector<uint8_t>& aPayload)
    {
        if (aPayload.empty())
        {
            return std::string{""};
        }

        std::string result(aPayload.begin(), aPayload.end());
        return result;
    }

    inline std::vector<uint8_t> StringToPayload(const std::string& aString)
    {
        if (aString.empty())
        {
            return std::vector<uint8_t>{};
        }

        std::vector<std::uint8_t> result(aString.begin(), aString.end());
        return result;
    }
}
