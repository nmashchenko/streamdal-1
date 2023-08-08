import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message protos.NotificationConfig
 */
export interface NotificationConfig {
    /**
     * @generated from protobuf field: optional string id = 1;
     */
    id?: string;
    /**
     * @generated from protobuf field: optional string name = 2;
     */
    name?: string;
    /**
     * @generated from protobuf field: optional protos.NotificationType type = 3;
     */
    type?: NotificationType;
    /**
     * @generated from protobuf oneof: config
     */
    config: {
        oneofKind: "slack";
        /**
         * @generated from protobuf field: protos.NotificationSlack slack = 1000;
         */
        slack: NotificationSlack;
    } | {
        oneofKind: "email";
        /**
         * @generated from protobuf field: protos.NotificationEmail email = 1001;
         */
        email: NotificationEmail;
    } | {
        oneofKind: "pagerduty";
        /**
         * @generated from protobuf field: protos.NotificationPagerDuty pagerduty = 1002;
         */
        pagerduty: NotificationPagerDuty;
    } | {
        oneofKind: undefined;
    };
}
/**
 * @generated from protobuf message protos.NotificationSlack
 */
export interface NotificationSlack {
    /**
     * @generated from protobuf field: optional string bot_token = 1;
     */
    botToken?: string;
    /**
     * @generated from protobuf field: optional string channel = 2;
     */
    channel?: string;
}
/**
 * @generated from protobuf message protos.NotificationEmail
 */
export interface NotificationEmail {
    /**
     * @generated from protobuf field: optional protos.NotificationEmail.Type type = 1;
     */
    type?: NotificationEmail_Type;
    /**
     * @generated from protobuf field: repeated string recipients = 2;
     */
    recipients: string[];
    /**
     * @generated from protobuf field: optional string from_address = 3;
     */
    fromAddress?: string;
    /**
     * @generated from protobuf oneof: config
     */
    config: {
        oneofKind: "smtp";
        /**
         * @generated from protobuf field: protos.NotificationEmailSMTP smtp = 1000;
         */
        smtp: NotificationEmailSMTP;
    } | {
        oneofKind: "ses";
        /**
         * @generated from protobuf field: protos.NotificationEmailSES ses = 1001;
         */
        ses: NotificationEmailSES;
    } | {
        oneofKind: undefined;
    };
}
/**
 * @generated from protobuf enum protos.NotificationEmail.Type
 */
export declare enum NotificationEmail_Type {
    /**
     * @generated from protobuf enum value: TYPE_UNSET = 0;
     */
    UNSET = 0,
    /**
     * @generated from protobuf enum value: TYPE_SMTP = 1;
     */
    SMTP = 1,
    /**
     * @generated from protobuf enum value: TYPE_SES = 2;
     */
    SES = 2
}
/**
 * @generated from protobuf message protos.NotificationEmailSMTP
 */
export interface NotificationEmailSMTP {
    /**
     * @generated from protobuf field: optional string smtp_host = 1;
     */
    smtpHost?: string;
    /**
     * @generated from protobuf field: optional int32 smtp_port = 2;
     */
    smtpPort?: number;
    /**
     * @generated from protobuf field: optional string smtp_user = 3;
     */
    smtpUser?: string;
    /**
     * @generated from protobuf field: optional string smtp_password = 4;
     */
    smtpPassword?: string;
    /**
     * @generated from protobuf field: optional bool use_tls = 5;
     */
    useTls?: boolean;
}
/**
 * @generated from protobuf message protos.NotificationEmailSES
 */
export interface NotificationEmailSES {
    /**
     * @generated from protobuf field: optional string ses_region = 1;
     */
    sesRegion?: string;
    /**
     * @generated from protobuf field: optional string ses_access_key_id = 2;
     */
    sesAccessKeyId?: string;
    /**
     * @generated from protobuf field: optional string ses_secret_access_key = 3;
     */
    sesSecretAccessKey?: string;
}
/**
 * @generated from protobuf message protos.NotificationPagerDuty
 */
export interface NotificationPagerDuty {
    /**
     * Auth token
     *
     * @generated from protobuf field: optional string token = 1;
     */
    token?: string;
    /**
     * Must be a valid email for a PagerDuty user
     *
     * @generated from protobuf field: optional string email = 2;
     */
    email?: string;
    /**
     * Must be a valid PagerDuty service
     *
     * @generated from protobuf field: optional string service_id = 3;
     */
    serviceId?: string;
    /**
     * @generated from protobuf field: protos.NotificationPagerDuty.Urgency urgency = 4;
     */
    urgency: NotificationPagerDuty_Urgency;
}
/**
 * @generated from protobuf enum protos.NotificationPagerDuty.Urgency
 */
export declare enum NotificationPagerDuty_Urgency {
    /**
     * @generated from protobuf enum value: URGENCY_UNSET = 0;
     */
    UNSET = 0,
    /**
     * @generated from protobuf enum value: URGENCY_LOW = 1;
     */
    LOW = 1,
    /**
     * @generated from protobuf enum value: URGENCY_HIGH = 2;
     */
    HIGH = 2
}
/**
 * @generated from protobuf enum protos.NotificationType
 */
export declare enum NotificationType {
    /**
     * @generated from protobuf enum value: NOTIFICATION_TYPE_UNSET = 0;
     */
    UNSET = 0,
    /**
     * @generated from protobuf enum value: NOTIFICATION_TYPE_SLACK = 1;
     */
    SLACK = 1,
    /**
     * @generated from protobuf enum value: NOTIFICATION_TYPE_EMAIL = 2;
     */
    EMAIL = 2,
    /**
     * @generated from protobuf enum value: NOTIFICATION_TYPE_PAGERDUTY = 3;
     */
    PAGERDUTY = 3
}
declare class NotificationConfig$Type extends MessageType<NotificationConfig> {
    constructor();
}
/**
 * @generated MessageType for protobuf message protos.NotificationConfig
 */
export declare const NotificationConfig: NotificationConfig$Type;
declare class NotificationSlack$Type extends MessageType<NotificationSlack> {
    constructor();
}
/**
 * @generated MessageType for protobuf message protos.NotificationSlack
 */
export declare const NotificationSlack: NotificationSlack$Type;
declare class NotificationEmail$Type extends MessageType<NotificationEmail> {
    constructor();
}
/**
 * @generated MessageType for protobuf message protos.NotificationEmail
 */
export declare const NotificationEmail: NotificationEmail$Type;
declare class NotificationEmailSMTP$Type extends MessageType<NotificationEmailSMTP> {
    constructor();
}
/**
 * @generated MessageType for protobuf message protos.NotificationEmailSMTP
 */
export declare const NotificationEmailSMTP: NotificationEmailSMTP$Type;
declare class NotificationEmailSES$Type extends MessageType<NotificationEmailSES> {
    constructor();
}
/**
 * @generated MessageType for protobuf message protos.NotificationEmailSES
 */
export declare const NotificationEmailSES: NotificationEmailSES$Type;
declare class NotificationPagerDuty$Type extends MessageType<NotificationPagerDuty> {
    constructor();
}
/**
 * @generated MessageType for protobuf message protos.NotificationPagerDuty
 */
export declare const NotificationPagerDuty: NotificationPagerDuty$Type;
export {};
