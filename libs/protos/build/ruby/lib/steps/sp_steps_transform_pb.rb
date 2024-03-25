# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: steps/sp_steps_transform.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("steps/sp_steps_transform.proto", :syntax => :proto3) do
    add_message "protos.steps.TransformStep" do
      optional :path, :string, 1
      optional :value, :string, 2
      optional :type, :enum, 3, "protos.steps.TransformType"
      oneof :options do
        optional :replace_value_options, :message, 101, "protos.steps.TransformReplaceValueOptions"
        optional :delete_field_options, :message, 102, "protos.steps.TransformDeleteFieldOptions"
        optional :obfuscate_options, :message, 103, "protos.steps.TransformObfuscateOptions"
        optional :mask_options, :message, 104, "protos.steps.TransformMaskOptions"
        optional :truncate_options, :message, 105, "protos.steps.TransformTruncateOptions"
        optional :extract_options, :message, 106, "protos.steps.TransformExtractOptions"
      end
    end
    add_message "protos.steps.TransformTruncateOptions" do
      optional :type, :enum, 1, "protos.steps.TransformTruncateType"
      optional :path, :string, 2
      optional :value, :int32, 3
    end
    add_message "protos.steps.TransformDeleteFieldOptions" do
      repeated :paths, :string, 1
    end
    add_message "protos.steps.TransformReplaceValueOptions" do
      optional :path, :string, 1
      optional :value, :string, 2
    end
    add_message "protos.steps.TransformObfuscateOptions" do
      optional :path, :string, 1
    end
    add_message "protos.steps.TransformMaskOptions" do
      optional :path, :string, 1
      optional :mask, :string, 2
    end
    add_message "protos.steps.TransformExtractOptions" do
      repeated :paths, :string, 1
      optional :flatten, :bool, 2
    end
    add_enum "protos.steps.TransformType" do
      value :TRANSFORM_TYPE_UNKNOWN, 0
      value :TRANSFORM_TYPE_REPLACE_VALUE, 1
      value :TRANSFORM_TYPE_DELETE_FIELD, 2
      value :TRANSFORM_TYPE_OBFUSCATE_VALUE, 3
      value :TRANSFORM_TYPE_MASK_VALUE, 4
      value :TRANSFORM_TYPE_TRUNCATE_VALUE, 5
      value :TRANSFORM_TYPE_EXTRACT, 6
    end
    add_enum "protos.steps.TransformTruncateType" do
      value :TRANSFORM_TRUNCATE_TYPE_UNKNOWN, 0
      value :TRANSFORM_TRUNCATE_TYPE_LENGTH, 1
      value :TRANSFORM_TRUNCATE_TYPE_PERCENTAGE, 2
    end
  end
end

module Streamdal
  module Protos
    TransformStep = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformStep").msgclass
    TransformTruncateOptions = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformTruncateOptions").msgclass
    TransformDeleteFieldOptions = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformDeleteFieldOptions").msgclass
    TransformReplaceValueOptions = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformReplaceValueOptions").msgclass
    TransformObfuscateOptions = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformObfuscateOptions").msgclass
    TransformMaskOptions = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformMaskOptions").msgclass
    TransformExtractOptions = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformExtractOptions").msgclass
    TransformType = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformType").enummodule
    TransformTruncateType = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("protos.steps.TransformTruncateType").enummodule
  end
end
