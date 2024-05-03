# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: steps/sp_steps_httprequest.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("steps/sp_steps_httprequest.proto", :syntax => :proto3) do
    add_message "protos.steps.HttpRequest" do
      optional :method, :enum, 1, "protos.steps.HttpRequestMethod"
      optional :url, :string, 2
      optional :body, :bytes, 3
      map :headers, :string, :string, 4
      optional :body_mode, :enum, 5, "protos.steps.HttpRequestBodyMode"
    end
    add_message "protos.steps.HttpResponse" do
      optional :code, :int32, 1
      optional :body, :bytes, 2
      map :headers, :string, :string, 3
    end
    add_message "protos.steps.HttpRequestStep" do
      optional :request, :message, 1, "protos.steps.HttpRequest"
    end
    add_enum "protos.steps.HttpRequestMethod" do
      value :HTTP_REQUEST_METHOD_UNSET, 0
      value :HTTP_REQUEST_METHOD_GET, 1
      value :HTTP_REQUEST_METHOD_POST, 2
      value :HTTP_REQUEST_METHOD_PUT, 3
      value :HTTP_REQUEST_METHOD_DELETE, 4
      value :HTTP_REQUEST_METHOD_PATCH, 5
      value :HTTP_REQUEST_METHOD_HEAD, 6
      value :HTTP_REQUEST_METHOD_OPTIONS, 7
    end
    add_enum "protos.steps.HttpRequestBodyMode" do
      value :HTTP_REQUEST_BODY_MODE_UNSET, 0
      value :HTTP_REQUEST_BODY_MODE_STATIC, 1
      value :HTTP_REQUEST_BODY_MODE_INTER_STEP_RESULT, 2
    end
  end
end

module Streamdal
  module Protos
    HttpRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.HttpRequest").msgclass
    HttpResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.HttpResponse").msgclass
    HttpRequestStep = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.HttpRequestStep").msgclass
    HttpRequestMethod = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.HttpRequestMethod").enummodule
    HttpRequestBodyMode = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.HttpRequestBodyMode").enummodule
  end
end
