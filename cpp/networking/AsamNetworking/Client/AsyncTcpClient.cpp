#include "pch.h"
#include "AsyncTcpClient.h"

#include "../Helpers.h"

//#include "payload_samples.h"

namespace asamNetworking::client
{
    using namespace boost;

    AsyncTcpClient::AsyncTcpClient(const unsigned char numOfThreads)
    {
        // instantiates an object of the asio::io_service::work class
        // passing an instance of the asio::io_service class named m_ios to its constructor
        work_.reset(new asio::io_service::work(ioService_));
        for (unsigned char i = 1; i <= numOfThreads; i++)
        {
            // spawns a thread that calls the run() method of the m_ios object.
            std::unique_ptr<std::thread> thread(new std::thread([this]
            {
                ioService_.run();
            }));

            threads_.push_back(std::move(thread));
        }
    }

    void AsyncTcpClient::ApplyRequest(
        const std::string& rawIpAddress,
        const unsigned short portNum,
        const Callback callback,
        const unsigned int requestId,
        const std::string& aRequest)
    {
        // TONOTE: Client's request processing
        // preparing a request string and allocating an instance of the Session structure
        // that keeps the data associated with the request including a socket object
        // that is used to communicate with the server.

        const std::string readyReq = aRequest + EndOfProtocolMarker;
        auto session = std::make_shared<Session>(ioService_, rawIpAddress, portNum, readyReq, requestId, callback);

        // opened socket and the pointer to the Session object is added to the m_active_sessions map
        session->Socket.open(session->Endpoint.protocol());

        // Add new session to the list of active sessions so
        // that we can access it if the user decides to cancel
        // the corresponding request before it completes.
        // Because active sessions list can be accessed from
        // multiple threads, we guard it with a mutex to avoid
        // data corruption.
        std::unique_lock<std::mutex> lock(activeSessionsGuard_);
        activeSessions_[requestId] = session;
        lock.unlock();

        // connect the socket to the server
        session->Socket.async_connect(
            session->Endpoint,
            [this, session](const system::error_code& errorCode)
            {
                // checking the error code passed to it as the ec argument
                if (errorCode.value() != 0)
                {
                    // we store the ec value in the corresponding Session object,
                    session->ErrorCode = errorCode;

                    // call the class's onRequestComplete() private method passing the Session object to it as an
                    // argument
                    OnRequestComplete(session);

                    // then return.
                    return;
                }

                // lock the m_cancel_guard mutex (the member of the request descriptor object)
                std::unique_lock<std::mutex> cancelLock(session->CancelGuard);

                // check whether the request has not been canceled yet.
                if (session->WasCancelled)
                {
                    OnRequestComplete(session);
                    return;
                }

                // If we see that the request has not been canceled
                // we initiate the next asynchronous operation calling the Boost.Asio free function async_write()
                // to send the request data to the server.
                asio::async_write(session->Socket,
                                  asio::buffer(session->Request),
                                  [this, session](const system::error_code& lErrorCode, std::size_t bytesTransferred)
                                  {
                                      // check the error code
                                      if (lErrorCode.value() != 0)
                                      {
                                          session->ErrorCode = lErrorCode;
                                          OnRequestComplete(session);
                                          return;
                                      }

                                      // lock the m_cancel_guard mutex (the member of the request descriptor object)
                                      std::unique_lock<std::mutex> lCancelLock(session->CancelGuard);

                                      // check whether the request has been canceled.
                                      if (session->WasCancelled)
                                      {
                                          OnRequestComplete(session);
                                          return;
                                      }

                                      // initiate the next asynchronous operation—async_read_until()—in order to receive a
                                      // response from the server
                                      asio::async_read_until(session->Socket,
                                                             session->ResponseBuf,
                                                             '\n',
                                                             [this, session](const system::error_code& llErrorCode, std::size_t transferred)
                                                             {
                                                                 // checks the error code
                                                                 if (llErrorCode.value() != 0)
                                                                 {
                                                                     session->ErrorCode = llErrorCode;
                                                                 }
                                                                 else
                                                                 {
                                                                     std::istream strm(&session->ResponseBuf);
                                                                     std::getline(strm, session->Response);
                                                                 }

                                                                 // the AsyncTCPClient class's private method onRequestComplete() is called
                                                                 // and the Session object is passed to it as an argument.
                                                                 OnRequestComplete(session);
                                                             });
                                  });
            });
    }

    void AsyncTcpClient::CancelRequest(const unsigned int requestId)
    {
        std::unique_lock<std::mutex> lock(activeSessionsGuard_);

        // looking for the Session object corresponding to the specified request in the m_active_sessions map.
        if (const auto iterator = activeSessions_.find(requestId); iterator != activeSessions_.end())
        {
            std::unique_lock<std::mutex> cancelLock(iterator->second->CancelGuard);
            iterator->second->WasCancelled = true;
            iterator->second->Socket.cancel();
        }
    }

    void AsyncTcpClient::Close()
    {
        // Destroy work object. This allows the I/O threads to
        // exit the event loop when there are no more pending
        // asynchronous operations.
        work_.reset(nullptr);

        // Waiting for the I/O threads to exit.
        for (const auto& thread : threads_)
        {
            thread->join();
        }
    }

    void AsyncTcpClient::OnRequestComplete(const std::shared_ptr<Session>& session)
    {
        // Shutting down the connection. This method may
        // fail in case socket is not connected. We don't care
        // about the error code if this function fails.
        system::error_code ignoredErrorCode;

        session->Socket.shutdown(asio::ip::tcp::socket::shutdown_both, ignoredErrorCode);

        // Remove session form the map of active sessions.
        std::unique_lock<std::mutex> lock(activeSessionsGuard_);

        if (const auto iterator = activeSessions_.find(session->Id); iterator != activeSessions_.end())
        {
            activeSessions_.erase(iterator);
        }

        lock.unlock();

        const system::error_code errorCode =
            session->ErrorCode.value() == 0 && session->WasCancelled ? asio::error::operation_aborted : session->ErrorCode;

        // Call the callback provided by the user.
        session->Callback(session->Id, session->Response, errorCode);
    }
}
