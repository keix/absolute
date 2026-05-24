import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as logs from 'aws-cdk-lib/aws-logs';
import * as apigwv2 from 'aws-cdk-lib/aws-apigatewayv2';
import * as integrations from 'aws-cdk-lib/aws-apigatewayv2-integrations';
import * as path from 'path';

export class ApiStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const tableName = 'system-calls';
    const functionName = 'system-calls-api';
    const apiName = 'system-calls-api';

    const table = new dynamodb.Table(this, 'Table', {
      tableName,
      partitionKey: { name: 'pk', type: dynamodb.AttributeType.STRING },
      sortKey: { name: 'sk', type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
      pointInTimeRecoverySpecification: {
        pointInTimeRecoveryEnabled: true,
      },
    });

    const logGroup = new logs.LogGroup(this, 'FunctionLogGroup', {
      logGroupName: `/aws/lambda/${functionName}`,
      retention: logs.RetentionDays.ONE_WEEK,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    const fn = new lambda.Function(this, 'Function', {
      functionName,
      runtime: lambda.Runtime.PROVIDED_AL2023,
      architecture: lambda.Architecture.ARM_64,
      handler: 'bootstrap',
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', '..', 'target', 'lambda', 'system-calls'),
      ),
      memorySize: 512,
      timeout: cdk.Duration.seconds(10),
      logGroup,
      environment: {
        DDB_TABLE: tableName,
        RUST_LOG: 'info,system_calls=info',
      },
    });

    table.grantReadData(fn);

    const httpApi = new apigwv2.HttpApi(this, 'HttpApi', {
      apiName,
      corsPreflight: {
        allowOrigins: ['*'],
        allowMethods: [apigwv2.CorsHttpMethod.GET],
        allowHeaders: ['*'],
      },
    });

    httpApi.addRoutes({
      path: '/{proxy+}',
      methods: [apigwv2.HttpMethod.GET],
      integration: new integrations.HttpLambdaIntegration('Lambda', fn),
    });

    new cdk.CfnOutput(this, 'ApiUrl', { value: httpApi.url ?? '' });
    new cdk.CfnOutput(this, 'TableName', { value: tableName });
    new cdk.CfnOutput(this, 'FunctionName', { value: functionName });
  }
}
