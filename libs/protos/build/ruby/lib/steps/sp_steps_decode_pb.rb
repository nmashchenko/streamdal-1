# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: steps/sp_steps_decode.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("steps/sp_steps_decode.proto", :syntax => :proto3) do
    add_message "protos.steps.DecodeStep" do
      optional :id, :string, 1
    end
  end
end

module Streamdal
  module Protos
    DecodeStep = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.DecodeStep").msgclass
  end
end
