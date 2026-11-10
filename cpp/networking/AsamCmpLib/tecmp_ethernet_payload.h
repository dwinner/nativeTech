#pragma once

#include "common.h"
#include "tecmp_payload.h"

BEGIN_NAMESPACE_TECMP
    class EthernetPayload : public Payload
    {
    public:
        EthernetPayload(const uint8_t* data, size_t size);

        const uint8_t* getData() const;

        std::size_t getDataLength() const;
    };

END_NAMESPACE_TECMP
