#include "payload_samples.h"

#include <numeric>
#include <vector>
#include "Helpers.h"

using namespace asamNetworking;

std::string CreateCanPayload()
{
    std::vector<uint8_t> data;
    constexpr size_t dataSize = 64;
    data.resize(dataSize);
    constexpr uint32_t arbId = 78;
    const auto message = CreateCanDataMessage(arbId, data);
    const auto canPayload = PayloadToString(message);

    return canPayload;
}

std::string CreateLinPayload()
{
    constexpr size_t dataSize = 40;
    constexpr uint8_t linId = 33;
    std::vector<uint8_t> data;
    data.resize(dataSize);
    std::iota(data.begin(), data.end(), uint8_t{});
    const std::vector<uint8_t> message = CreateLinDataMessage(linId, data);
    const auto linPayload = PayloadToString(message);

    return linPayload;
}

std::string CreateEthPayload()
{
    constexpr size_t dataSize = 40;
    std::vector<uint8_t> data;
    data.resize(dataSize);
    std::iota(data.begin(), data.end(), uint8_t{});
    const std::vector<uint8_t> message = CreateEthernetDataMessage(data);
    const auto ethPayload = PayloadToString(message);

    return ethPayload;
}
