# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sp_command.proto

require 'google/protobuf'

require 'shared/sp_shared_pb'
require 'sp_common_pb'
require 'sp_kv_pb'
require 'sp_pipeline_pb'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("sp_command.proto", :syntax => :proto3) do
    add_message "protos.Command" do
      optional :audience, :message, 1, "protos.Audience"
      oneof :command do
        optional :set_pipelines, :message, 100, "protos.SetPipelinesCommand"
        optional :keep_alive, :message, 101, "protos.KeepAliveCommand"
        optional :kv, :message, 102, "protos.KVCommand"
        optional :tail, :message, 103, "protos.TailCommand"
        optional :delete_audiences, :message, 104, "protos.DeleteAudiencesCommand"
      end
    end
    add_message "protos.SetPipelinesCommand" do
      repeated :pipelines, :message, 1, "protos.Pipeline"
      map :wasm_modules, :string, :message, 2, "protos.shared.WasmModule"
    end
    add_message "protos.KeepAliveCommand" do
    end
    add_message "protos.KVCommand" do
      repeated :instructions, :message, 1, "protos.KVInstruction"
      optional :overwrite, :bool, 2
    end
    add_message "protos.TailCommand" do
      optional :request, :message, 2, "protos.TailRequest"
    end
    add_message "protos.DeleteAudiencesCommand" do
      repeated :audiences, :message, 1, "protos.Audience"
    end
  end
end

module Streamdal
  module Protos
    Command = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.Command").msgclass
    SetPipelinesCommand = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.SetPipelinesCommand").msgclass
    KeepAliveCommand = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.KeepAliveCommand").msgclass
    KVCommand = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.KVCommand").msgclass
    TailCommand = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.TailCommand").msgclass
    DeleteAudiencesCommand = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.DeleteAudiencesCommand").msgclass
  end
end
