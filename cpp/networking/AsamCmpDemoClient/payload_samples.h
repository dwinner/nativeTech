#pragma once

#include <string>

#include "can_payload.h"
#include "ethernet_payload.h"
#include "lin_payload.h"

std::string CreateCanPayload();

std::string CreateLinPayload();

std::string CreateEthPayload();

template <typename Header>
std::vector<uint8_t> CreateMessage(Header&& header, const std::vector<uint8_t>& data)
{
    const size_t messageSize = sizeof(header) + data.size();
    std::vector<uint8_t> message(messageSize);
    memcpy(message.data(), &header, sizeof(header));
    std::copy(data.begin(), data.end(), std::next(message.begin(), sizeof(header)));

    return message;
}

inline std::vector<uint8_t> CreateCanDataMessage(const uint32_t arbId, const std::vector<uint8_t>& data)
{
    ASAM::CMP::CanPayload::Header header;
    header.setId(arbId);
    header.setDataLength(static_cast<uint8_t>(data.size()));
    constexpr uint8_t dlc = 0x0F;
    header.setDlc(dlc);

    return CreateMessage(header, data);
}

inline std::vector<uint8_t> CreateLinDataMessage(const uint8_t linId, const std::vector<uint8_t>& data)
{
    ASAM::CMP::LinPayload::Header header;
    header.setLinId(linId);
    header.setDataLength(static_cast<uint8_t>(data.size()));

    return CreateMessage(header, data);
}

inline std::vector<uint8_t> CreateEthernetDataMessage(const std::vector<uint8_t>& data)
{
    ASAM::CMP::EthernetPayload::Header header;
    header.setDataLength(static_cast<uint16_t>(data.size()));

    return CreateMessage(header, data);
}
