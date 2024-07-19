class User {
    public readonly id: String;
    public readonly name: {
        first: String,
        last: String
    }
    public readonly email: String;
    public readonly createdAt: Date;
    public readonly updatedAt: Date;

    public constructor({
        id,
        name,
        email,
        createdAt,
        updatedAt
    }: {
        id: String,
        name: {
            first: String,
            last: String
        }
        email: String,
        createdAt: Date,
        updatedAt: Date
    }) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        return this;
    }
}

export {
    User
}