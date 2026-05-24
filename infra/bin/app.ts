#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { ApiStack } from '../lib/api-stack';

const app = new cdk.App();

const envName = (app.node.tryGetContext('env') as string | undefined) ?? 'dev';
if (!/^[a-z][a-z0-9-]{0,15}$/.test(envName)) {
  throw new Error(`invalid env name: ${envName}`);
}

new ApiStack(app, `SystemCallsApi-${envName}`, {
  envName,
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION ?? 'ap-northeast-1',
  },
});
