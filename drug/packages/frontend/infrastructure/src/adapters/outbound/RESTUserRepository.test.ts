import path from "path";
import { expect, test, describe } from "vitest";
import { User } from "@stitchmate/user-domain";
import RESTUserRepository from "./RESTUserRepository";
import {
  PactV4,
  SpecificationVersion,
  MatchersV3,
} from "@pact-foundation/pact";

describe("RESTUserRepository", () => {
  const provider = new PactV4({
    dir: path.resolve(process.cwd(), "pacts"),
    consumer: "RESTUserConsumer",
    provider: "RESTUserProvider",
    spec: SpecificationVersion.SPECIFICATION_VERSION_V4,
  });

  test("successfully gets an existing user", async () => {
    const id = "1";
    const expected: User = new User({
      id,
      name: {
        first: "Test",
        last: "User",
      },
      email: "test@test.com",
      createdAt: (new Date()),
      updatedAt: (new Date()),
    });

    const interaction = provider
      .addInteraction()
      .given("User 1 exists", { user: MatchersV3.like(expected)["pact:matcher:type"] })
      .uponReceiving("a request to get a user")
      .withRequest("GET", `/user/${id}`)
      .willRespondWith(200, (builder) => {
        builder
          .headers({
            "Content-Type": "application/json",
          })
          .jsonBody(expected);
      });
    
    return interaction.executeTest(async (mockServer) => {
      //Arrange
      const repository = new RESTUserRepository();
      repository.setBaseUrl(mockServer.url);

      //Act
      const result = await repository.get(id);

      //Assert
      expect(result.isOk()).toBe(true);
      expect(result.unwrap()).toEqual(expected);
    })
  });
});
