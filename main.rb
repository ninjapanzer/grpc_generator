$LOAD_PATH << File.expand_path('gen')

# The service is an interface for the methods an implementation has to meet
require 'sample_services_pb'


# Offline is an implementation of the BookService and allows for get_book to be called
class Offline < BookService::Service
  def get_book(request, _call)
    Book.new(
      isbn: request.isbn,
      title: "title",
      author: "author"
    )
  end
end

# We register a single instance of our service for use with Dependency Injection
off = Offline.new

# Construct a request based on the GetBookRequest Message
request = GetBookRequest.new(isbn: 1234)

# call through RPC without GRPC the implementation
resp = off.get_book(request, nil)

# Evaluate the response
puts resp.isbn
puts resp.title